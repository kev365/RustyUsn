# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.6.0] - 2025-02-04
### Changed (Kevin Stokes)
 - updating to support v3 128-bit references
 - Adjusting some output (i.e. shorter reason info)

## [1.5.0] - 2019-01-07
### Changed
 - updated to mft 0.5

## [1.5.0] - 2019-12-01
### Added
 - Major improvements for file path enumeration for listen_usn (now v1.1.0)
 - Minor code cleanup

## [1.4.0] - 2019-07-25
### Added
 - Version 3 records for live monitoring and full path resolution for monitor.
 
## [1.3.0] - 2019-07-25
### Added
 - added print_folder_mapping example tool
 - added path enumeration to rusty_usn tool (now v1.2.0)

### Changed
 - file attributes are now flags and not a integer

## [1.2.0] - 2019-06-14
### Added
 - listen_usn tool (0.1.0)

### Changed
 - changed Windows build scripts for azure pipeline

## [1.1.0] - 2019-06-01
### Added
 - `_source` to output
 - directory processing

## [1.0.0] - 2019-05-27
### Changed
- Rewrite and removal of features