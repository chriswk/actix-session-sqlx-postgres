# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4](https://github.com/chriswk/actix-session-sqlx-postgres/compare/v0.1.3...v0.1.4) - 2023-07-03

### Added
- from pool needs not be async

## [0.1.3](https://github.com/chriswk/actix-session-sqlx-postgres/compare/v0.1.2...v0.1.3) - 2023-07-03

### Fixed
- Make connection Arced

## [0.1.2](https://github.com/chriswk/actix-session-sqlx-postgres/compare/v0.1.1...v0.1.2) - 2023-06-30

### Fixed
- cargo update

### Other
- Bump dependencies

## [0.1.1](https://github.com/chriswk/actix-session-sqlx-postgres/compare/v0.1.0...v0.1.1) - 2023-05-11

### Added
- Postgresql Session Store for Actix

### Fixed
- Use correct license identificator
- Updated to use jsonb type for storing session data

### Other
- release
- Drop minimal test run
- Set description and license
- set MSRV to 1.69.0
- Added test workflow
- Add acceptance test suite
- Initial commit

## [0.1.0](https://github.com/chriswk/actix-session-sqlx-postgres/releases/tag/v0.1.0) - 2023-05-11

### Added
- Postgresql Session Store for Actix

### Fixed
- Use correct license identificator
- Updated to use jsonb type for storing session data

### Other
- Drop minimal test run
- Set description and license
- set MSRV to 1.69.0
- Added test workflow
- Add acceptance test suite
- Initial commit

## v0.3.0 (2024-09-20)

### Chore

 - <csr-id-96bf785336e313c926a2dc77f5c1e5c626b57f5f/> Update cargo file

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update cargo file ([`96bf785`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/96bf785336e313c926a2dc77f5c1e5c626b57f5f))
    - Merge pull request #13 from chriswk/11-incompatible-with-latest-sqlxactix-session ([`64d8f1b`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/64d8f1b1e1c64504a810826715d4185978ea2c51))
    - Updated to latest actix-session and sqlx ([`a454500`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/a454500a7fd0050e5a870926a5967079d196237d))
</details>

## v0.1.4 (2023-07-03)

### Chore

 - <csr-id-1492e5bc4cffa7ffb0fcdc766a09d92e0db6421a/> release

### New Features

 - <csr-id-2a6253047a48971c0c0cf0589aa71176f53e6a30/> from pool needs not be async

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #8 from chriswk/release-plz/2023-07-03T09-28-45Z ([`82f9d73`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/82f9d7331ac78877da302bb34892fb81f062e696))
    - Release ([`1492e5b`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/1492e5bc4cffa7ffb0fcdc766a09d92e0db6421a))
    - From pool needs not be async ([`2a62530`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/2a6253047a48971c0c0cf0589aa71176f53e6a30))
</details>

## v0.1.3 (2023-07-03)

### Chore

 - <csr-id-c081ab343437c5b106ea82b72c96a10861df011b/> release

### Bug Fixes

 - <csr-id-5da70d23dd52baa763b3020b27c697f46ff9c7e3/> Make connection Arced

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #7 from chriswk/release-plz/2023-07-03T09-17-39Z ([`8165a7c`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/8165a7c9d0abf9f11022497195f4e23335af8485))
    - Release ([`c081ab3`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/c081ab343437c5b106ea82b72c96a10861df011b))
    - Make connection Arced ([`5da70d2`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/5da70d23dd52baa763b3020b27c697f46ff9c7e3))
</details>

## v0.1.2 (2023-06-30)

### Chore

 - <csr-id-c23097922529aeaa8a8526046da51fb6448cbc50/> release

### Bug Fixes

 - <csr-id-c9a2db4737ed3650a3250cbd747a148daff4a1d0/> cargo update

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #6 from chriswk/release-plz/2023-06-30T13-29-30Z ([`23fbc4f`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/23fbc4f87aa46c017d5d1e2778e8d9d7a6988305))
    - Release ([`c230979`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/c23097922529aeaa8a8526046da51fb6448cbc50))
    - Cargo update ([`c9a2db4`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/c9a2db4737ed3650a3250cbd747a148daff4a1d0))
    - Bump dependencies ([`5cda890`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/5cda890be656429b550fab0ed7d7048d910926fd))
</details>

## v0.1.1 (2023-06-30)

### Chore

 - <csr-id-05fecef4f48d5da5a226557532d56dc37abf9659/> release
 - <csr-id-b1d5added244a1eb0ad4203382e6d369bb4c29ad/> release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #5 from chriswk/release-plz/2023-05-11T08-14-18Z ([`f072117`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/f072117e94784433c51e9bcdff6fa14c7b4ea3c5))
    - Release ([`05fecef`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/05fecef4f48d5da5a226557532d56dc37abf9659))
    - Merge pull request #4 from chriswk/release-plz/2023-05-11T07-05-23Z ([`0680cb3`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/0680cb30eedf8307ac39296d65ae6d4722a15881))
    - Release ([`b1d5add`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/b1d5added244a1eb0ad4203382e6d369bb4c29ad))
</details>

## v0.1.0 (2023-05-11)

### Documentation

 - <csr-id-14837fa6ab777fdd5c70b9d636f0bf01bfaa304e/> Set description and license

### New Features

 - <csr-id-3fc0e6e22785ce57c37e501ab0a229ad4b413ef2/> Postgresql Session Store for Actix

### Bug Fixes

 - <csr-id-83fc00ee96a5fe302fc63a274f42956410edf3d5/> Use correct license identificator
 - <csr-id-0d0aebb04bf34d638775e8b96697f3f746723d4a/> Updated to use jsonb type for storing session data

### Other

 - <csr-id-2c5feb18ab14f5bf77c81f7477a84146420f7ae2/> Drop minimal test run
 - <csr-id-68f0541787a25488f566a645f70a23fc0ad53918/> set MSRV to 1.69.0
 - <csr-id-4df78e9c8f551693051197c7e2c28329e000cafa/> Added test workflow
 - <csr-id-a725dd267c5801f72a934ca0b1eee2897261c47e/> Add acceptance test suite

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Use correct license identificator ([`83fc00e`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/83fc00ee96a5fe302fc63a274f42956410edf3d5))
    - Drop minimal test run ([`2c5feb1`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/2c5feb18ab14f5bf77c81f7477a84146420f7ae2))
    - Set description and license ([`14837fa`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/14837fa6ab777fdd5c70b9d636f0bf01bfaa304e))
    - Set MSRV to 1.69.0 ([`68f0541`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/68f0541787a25488f566a645f70a23fc0ad53918))
    - Added test workflow ([`4df78e9`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/4df78e9c8f551693051197c7e2c28329e000cafa))
    - Updated to use jsonb type for storing session data ([`0d0aebb`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/0d0aebb04bf34d638775e8b96697f3f746723d4a))
    - Add acceptance test suite ([`a725dd2`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/a725dd267c5801f72a934ca0b1eee2897261c47e))
    - Postgresql Session Store for Actix ([`3fc0e6e`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/3fc0e6e22785ce57c37e501ab0a229ad4b413ef2))
    - Initial commit ([`f8282bb`](https://github.com/chriswk/actix-session-sqlx-postgres/commit/f8282bb73bdb102937559442f80d5931466f7cdd))
</details>

