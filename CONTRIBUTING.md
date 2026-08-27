# Contributing to suite-common-rust

> ⚠️ **Notice: Deprecated Repository**
> `suite-common-rust` has been superseded by the [`gtk-office-suite`](https://github.com/tuna-os/gtk-office-suite) monorepo (`gtk-office-suite/suite-common/`).
> New features, expanded APIs, and core enhancements should be submitted directly to `gtk-office-suite`.

## Maintenance & Fixes

If submitting critical maintenance fixes to this legacy repository:

### Local Verification

This crate provides shared GTK4 and libadwaita UI components for Rust applications.

To run tests locally:
```bash
cargo test
```

> **Note**: Unit tests initialize GTK4 / libadwaita components (`make_app`, `make_header_bar`, `make_toolbar`, `is_dark_mode`) and require a valid GTK4 display environment. In headless or CI environments, ensure display mocking/initialization checks succeed.

### Commit Guidelines & DCO

All contributions must include a Developer Certificate of Origin (DCO) sign-off line:
```bash
git commit -s -m "docs: add details for contributor guidelines"
```
