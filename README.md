<p align="center">
  <h1 align="center"><a href="https://dublette.espadat.com">Dublette</a></h1>
</p>
<p align="center">
  <a href="https://crates.io/crates/dublette">
    <img src="https://img.shields.io/crates/v/dublette" alt="Crates.io">
  </a>
</p>

> Find the same photo, video or song saved twice, even when the files differ.

Dublette scans a folder for media that looks or sounds the same: a photo saved again at another size, a re-encoded video, one album as both mp3 and flac. Byte-level duplicate finders miss all of these. Dublette compares content instead, with perceptual hashes for images and video and acoustic fingerprints for audio, and keeps one copy per group. Run it with `--dry-run` first to see what it would delete.

## Features

- **Perceptual hashing** -- detects visually similar images and videos, not just byte-identical copies
- **Acoustic fingerprinting** -- groups the same recording across mp3, flac, and 7 more audio formats, keeping the highest-fidelity copy
- **Image and video support** -- handles jpg, png, gif, webp, bmp, tiff, and 9 video formats via ffmpeg
- **Dry-run mode** -- preview what would be deleted before committing
- **JSON output** -- machine-readable output for scripting and CI pipelines
- **Parallel processing** -- hashes files concurrently using all available cores
- **Configurable threshold** -- tune sensitivity with hamming distance control

## Quick Start

Install dublette from a [pre-compiled binary](https://github.com/espadat-studio/dublette/releases/latest) or via cargo:

```bash
cargo install dublette
```

Preview duplicates:

```bash
dublette ~/Photos --dry-run
```

Delete duplicates:

```bash
dublette ~/Photos --yes
```

## Documentation

Full documentation available at [dublette.espadat.com](https://dublette.espadat.com):

- [Installation](https://dublette.espadat.com/getting-started/installation/) - Detailed setup guide
- [Quick Start](https://dublette.espadat.com/getting-started/quick-start/) - Step-by-step walkthrough
- [CLI Reference](https://dublette.espadat.com/cli-reference/) - All options documented
- [How It Works](https://dublette.espadat.com/how-it-works/perceptual-hashing/) - Perceptual hashing explained

## Requirements

- (Optional) ffmpeg for video deduplication and audio recording match

## Community

- [Documentation](https://dublette.espadat.com)
- [Report Issues](https://github.com/espadat-studio/dublette/issues)
