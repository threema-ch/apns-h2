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

- ...

### [v0.11.0-alpha.1][v0.11.0-alpha.1] (2026-01-28)

- [changed] Replace usage of &'a str with Cow

### [v0.11.0-alpha.0][v0.11.0-alpha.0] (2026-01-26)

- [changed] Remove lockfile
- [changed] Migrate to edition 2024 and set msrv to 1.85
- [changed] Migrate from `rustls-pemfile` to `rustls-pki-types`
- [changed] Rename crate to `apns-h2`
- [changed] Make ring the default crypto provider
- [changed] Clean up features of dependencies
- [added] Add pure Rust PKCS#12 parsing with p12-keystore
- [changed] Update `erased-serde`, `thiserror`, `base64`, `hyper-rustls` and `rustls` deps
- [added] Keep http connections open
- [added] Support thread-id
- [added] Support interruption-level
- [added] Support Live activity
- [added] Support Dismissal date
- [added] Add all current APNs errors
- [changed] Make `hyper-rustls` use `rustls-platform-verifier`
- [changed] Change APNs development endpoint to api.sandbox.push.apple.com
- [changed] Mark `set_` functions on builder as deprecated
- [added] Support for subtitle-loc-(key|args) in alert
- [changed] Remove support for legacy body-only alert notification

[0.11.0-alpha.1]: https://github.com/threema-ch/apns-h2/compare/v0.11.0-alpha.0...v0.11.0-alpha.1
[0.11.0-alpha.0]: https://github.com/threema-ch/apns-h2/compare/v0.10.0...v0.11.0-alpha.0
