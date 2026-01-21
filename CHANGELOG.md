# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### Unreleased

- [changed] Remove lockfile
- [changed] Migrate to edition 2024 and set msrv to 1.85
- [changed] Migrate from `rustls-pemfile` to `rustls-pki-types`
- [changed] Rename crate to `apns-h2`
- [changed] Make ring the default crypto provider
- [changed] Clean up features of dependencies
- [added] Add pure Rust PKCS#12 parsing with p12-keystore
