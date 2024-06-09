# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

[comment]: <> (Added:      new features)
[comment]: <> (Changed:    changes in existing functionality)
[comment]: <> (Deprecated: soon-to-be removed features)
[comment]: <> (Removed:    now removed features)
[comment]: <> (Fixed:      any bug fixes)
[comment]: <> (Security:   in case of vulnerabilities)

## [2.1.0] - 2024-06-10

## Added

- Add support for Windows 10 and greater.
- Add support for `TERM` environment variable: if set to `dumb` disables
  coloured output, if set to any other value enables coloured output. `NO_COLOR`
  `CLICOLOR_FORCE` both override this behaviour.
- Add changelog.

## [2.0.0] - 2024-05-13

### Added

- Add support for `std::fmt::Formatter` in `write_` macros.
- Environment variable `NO_COLOR` if set to any non-empty value will cause
  colouring to be disabled as recommended in [no-color.org] and
  [Standard for ANSI Colors in Terminals].
- Environment variable `CLICOLOR_FORCE` if set to any non-empty value will cause
  colouring to be enabled as recommended in
  [Standard for ANSI Colors in Terminals]. `NO_COLOR` overrides
  `CLICOLOR_FORCE`.
- If neither of the above environment variables are set to a non-empty value,
  colouring will only be enabled if `stdout` is detected to be a terminal or
  tty.
- Add `force_colour` function to force coloured output, regardless of
  environment variables or terminal detection.
- Add `force_no_colour` function to disable coloured output, regardless of
  environment variables or terminal detection.
- Add `force_color` and `force_no_color` as aliases for above functions to
  accommodate ~~incorrect~~ US spelling.
- Add `gray_` macros, equivalent to `grey_` ones to accommodate ~~wrong~~ US
  spelling.
- Specify minimum supported Rust version as 1.70.0.
- Add examples.

### Removed

- Remove dependency on `crossterm`, breaking Windows support, since `crossterm`
  cannot support writing using `std::fmt::Formatter`.

## [1.1.0] - 2024-04-29

### Changed

- Avoid consuming writer in `write_` macros.

## [1.0.0] - 2024-04-28

### Added

- Provide coloured write macros, similar to `write!` from the Standard Library.
- Add Cargo.lock to repo.

### Changed

- Update `crossterm` version.

## [0.7.0] - 2023-03-02

### Changed

- Refactor macro implementation to use a higher level macro to generate the individual public macros.
- Update to edition 2021.
- Update `crossterm` version.

## [0.6.0] - 2021-03-15

### Changed

- Change to MIT and Apache 2.0 licenses.
- Enable `clippy::pedantic` warnings.
- Update `crossterm` version.

## [0.5.0] - 2020-03-18

### Added

- Add macros prefixed with `e_` for writing to `stderr`.

## [0.4.0] - 2020-02-25

### Changed

- Swap `term` crate for `crossterm`.
- Renamed `dark_white` macros to `grey` in line with `crossterm` naming.
- Renamed `dark_black` macros to `black` in line with `crossterm` naming.
- Renamed `black` macros to `dark_grey` in line with `crossterm` naming.

## [0.3.1] - 2020-02-25

### Changed

- Update `term` version.
- Update formatting options.

## [0.3.0] - 2019-04-12

### Changed

- Update `lazy_static` version.
- Update `term` version.

## [0.2.1] - 2018-02-01

### Changed

- Update `lazy_static` version.

## [0.2.0] - 2017-04-09

### Added

- Add `allow(useless_format)` to disable Clippy lint.

## [0.1.0] - 2017-04-09

### Added

- Initial set of coloured macros.

[unreleased]: https://github.com/Fraser999/Colour/compare/v2.0.0...HEAD
[2.1.0]: https://github.com/Fraser999/Colour/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/Fraser999/Colour/compare/v1.1.0...v2.0.0
[1.1.0]: https://github.com/Fraser999/Colour/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/Fraser999/Colour/compare/v0.7.0...v1.0.0
[0.7.0]: https://github.com/Fraser999/Colour/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/Fraser999/Colour/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/Fraser999/Colour/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/Fraser999/Colour/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/Fraser999/Colour/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/Fraser999/Colour/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/Fraser999/Colour/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Fraser999/Colour/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Fraser999/Colour/releases/tag/v0.1.0

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[Standard for ANSI Colors in Terminals]: http://bixense.com/clicolors
[no-color.org]: https://no-color.org
