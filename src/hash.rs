use std::path::{Path, PathBuf};
use std::process::Command;

use image_hasher::{HashAlg, HasherConfig, ImageHash};

use crate::skip::{SkipError, SkipReason};

fn hasher() -> image_hasher::Hasher {
    HasherConfig::new()
        .hash_alg(HashAlg::DoubleGradient)
        .hash_size(8, 8)
        .to_hasher()
}

pub fn compute_image_hash(path: &Path, ffmpeg: Option<&Path>) -> Result<ImageHash, SkipError> {
    let error = match image::open(path) {
        Ok(img) => return Ok(hasher().hash_image(&img)),
        Err(error) => error,
    };

    if let Some(img) = ffmpeg.and_then(|ffmpeg| ffmpeg_decode_image(path, ffmpeg)) {
        return Ok(hasher().hash_image(&img));
    }

    let reason = if matches!(error, image::ImageError::IoError(_)) {
        SkipReason::Unreadable
    } else {
        SkipReason::DecodeFailed
    };
    Err(SkipError::new(
        reason,
        format!("failed to open {}: {error}", path.display()),
    ))
}

pub fn find_ffmpeg() -> eyre::Result<PathBuf> {
    which::which("ffmpeg").map_err(|_| eyre::eyre!("ffmpeg not found on PATH"))
}

fn run_ffmpeg_extract(ffmpeg: &Path, video: &Path, seek: &str, output: &Path) -> bool {
    Command::new(ffmpeg)
        .args([
            "-y",
            "-ss",
            seek,
            "-i",
            &video.to_string_lossy(),
            "-frames:v",
            "1",
            &output.to_string_lossy(),
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn ffmpeg_decode_image(path: &Path, ffmpeg: &Path) -> Option<image::DynamicImage> {
    let tmp = tempfile::NamedTempFile::new().ok()?.into_temp_path();
    let output = tmp.to_path_buf().with_extension("png");

    let decoded = run_ffmpeg_extract(ffmpeg, path, "0", &output)
        .then(|| image::open(&output).ok())
        .flatten();

    let _ = std::fs::remove_file(&output);
    decoded
}

pub fn extract_video_frame_hash(video: &Path, ffmpeg: &Path) -> Result<ImageHash, SkipError> {
    let tmp = tempfile::NamedTempFile::new()?.into_temp_path();
    let frame_path = tmp.to_path_buf().with_extension("png");

    for seek in &["1", "0"] {
        if run_ffmpeg_extract(ffmpeg, video, seek, &frame_path)
            && frame_path.exists()
            && std::fs::metadata(&frame_path)?.len() > 0
        {
            let img = image::open(&frame_path).map_err(|e| {
                SkipError::new(
                    SkipReason::DecodeFailed,
                    format!("failed to open extracted frame: {e}"),
                )
            })?;
            let _ = std::fs::remove_file(&frame_path);
            return Ok(hasher().hash_image(&img));
        }
    }

    let _ = std::fs::remove_file(&frame_path);
    Err(SkipError::new(
        SkipReason::DecodeFailed,
        format!("ffmpeg could not extract frame from {}", video.display()),
    ))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use image::{ImageBuffer, Rgb, RgbImage};

    use super::*;

    fn create_gradient_image(path: &PathBuf, horizontal: bool) {
        let img: RgbImage = ImageBuffer::from_fn(100, 100, |x, y| {
            let val = if horizontal { x as u8 } else { y as u8 };
            Rgb([val, val, val])
        });
        img.save(path).unwrap();
    }

    fn create_checkerboard_image(path: &PathBuf, block_size: u32) {
        let img: RgbImage = ImageBuffer::from_fn(100, 100, |x, y| {
            if ((x / block_size) + (y / block_size)) % 2 == 0 {
                Rgb([255, 255, 255])
            } else {
                Rgb([0, 0, 0])
            }
        });
        img.save(path).unwrap();
    }

    #[test]
    fn identical_images_same_hash() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.png");
        let b = dir.path().join("b.png");
        create_gradient_image(&a, true);
        create_gradient_image(&b, true);

        let hash_a = compute_image_hash(&a, None).unwrap();
        let hash_b = compute_image_hash(&b, None).unwrap();
        assert_eq!(hash_a.dist(&hash_b), 0);
    }

    #[test]
    fn different_images_different_hash() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.png");
        let b = dir.path().join("b.png");
        create_gradient_image(&a, true);
        create_checkerboard_image(&b, 10);

        let hash_a = compute_image_hash(&a, None).unwrap();
        let hash_b = compute_image_hash(&b, None).unwrap();
        assert!(hash_a.dist(&hash_b) > 0);
    }

    #[test]
    fn nonexistent_file_errors() {
        let result = compute_image_hash(Path::new("/nonexistent.png"), None);
        assert!(result.is_err());
    }

    #[test]
    fn malformed_jpeg_falls_back_to_ffmpeg() {
        let Ok(ffmpeg) = find_ffmpeg() else { return };

        let dir = tempfile::tempdir().unwrap();
        let valid = dir.path().join("valid.jpg");
        let img: RgbImage = ImageBuffer::from_fn(32, 32, |x, y| Rgb([x as u8 * 8, y as u8 * 8, 0]));
        img.save(&valid).unwrap();

        let bytes = std::fs::read(&valid).unwrap();
        let mut malformed = vec![0xFF, 0xD8, 0xFF, 0xD1];
        malformed.extend_from_slice(&bytes[2..]);
        let malformed_path = dir.path().join("malformed.jpg");
        std::fs::write(&malformed_path, malformed).unwrap();

        assert!(
            compute_image_hash(&malformed_path, None).is_err(),
            "image crate should reject the malformed JPEG"
        );
        assert!(
            compute_image_hash(&malformed_path, Some(&ffmpeg)).is_ok(),
            "ffmpeg fallback should recover the malformed JPEG"
        );
    }

    #[test]
    fn ffmpeg_not_at_path_errors() {
        let result = extract_video_frame_hash(
            Path::new("/nonexistent.mp4"),
            Path::new("/nonexistent/ffmpeg"),
        );
        assert!(result.is_err());
    }

    #[test]
    fn find_ffmpeg_succeeds_when_installed() {
        if which::which("ffmpeg").is_ok() {
            assert!(find_ffmpeg().is_ok());
        }
    }
}
