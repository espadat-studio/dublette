---
title: "Dublette"
description: "Find the same photo, video or song saved twice in another size, format or bitrate. Compares content, not bytes."
---

> Find the same photo, video or song saved twice, even when the files differ.

Dublette scans a folder for media that looks or sounds the same: a photo saved again at another size, a re-encoded video, one album as both mp3 and flac. Byte-level duplicate finders miss all of these. Dublette compares content instead, with perceptual hashes for images and video and acoustic fingerprints for audio, and keeps one copy per group. Run it with `--dry-run` first to see what it would delete.

## Key Features

- **Perceptual hashing** -- detects visually similar images and videos, not just byte-identical copies
- **Acoustic fingerprinting** -- groups the same recording across mp3, flac, and 7 more audio formats, keeping the highest-fidelity copy
- **Image and video support** -- handles jpg, png, gif, webp, bmp, tiff, and 9 video formats via ffmpeg
- **Dry-run mode** -- preview what would be deleted before committing
- **JSON output** -- machine-readable output for scripting and CI pipelines
- **Parallel processing** -- hashes files concurrently using all available cores
- **Configurable threshold** -- tune sensitivity with hamming distance control

## Quick Example

Preview duplicates without deleting anything:

```bash
dublette ~/Photos --dry-run
```

Delete duplicates, skipping the confirmation prompt:

```bash
dublette ~/Photos --yes
```

Output results as JSON for scripting:

```bash
dublette ~/Photos --dry-run --json
```

## Getting Started

See [Installation](/getting-started/installation/) to install dublette, then follow the [Quick Start](/getting-started/quick-start/) guide.
