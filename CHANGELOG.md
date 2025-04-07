# Changelog
<!--
SPDX-FileCopyrightText: 2025 The vita49-rs Authors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.3] - 2025-04-07

### Added

- Full support for CIF1 threshold field

### Changed

- Fixed various bit arithmetic bugs
- Fixed build error in benchmark app
- Various small CI fixes/improvements

### Removed

- `window_time_delta_ns()` accessor removed - replaced by `window_time_delta()`
- Visibility of `set_tsi()` and `set_tsf()` methods limited
- Binary test data replaced by JSON representations

## [0.0.2] - 2025-03-14

### Added

- Initial crate release.
- Basic documentation, test, and examples.

[0.0.3]: https://github.com/voyager-tech-inc/vita49-rs/releases/tag/0.0.3
[0.0.2]: https://github.com/voyager-tech-inc/vita49-rs/releases/tag/0.0.2
[0.0.1]: Unreleased
