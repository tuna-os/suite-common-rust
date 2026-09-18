# Runbook: `suite-common-rust` Deprecation, Maintenance & Monorepo Migration

## Overview

`suite-common-rust` is a legacy, standalone crate containing shared GTK4 and libadwaita UI scaffolding routines (`make_app`, `make_header_bar`, `make_toolbar`, `is_dark_mode`). 

The canonical `suite-common` implementation has migrated to the [`gtk-office-suite`](https://github.com/tuna-os/gtk-office-suite) monorepo (`gtk-office-suite/suite-common/`).

This runbook outlines operational procedures for managing dependency updates, legacy maintenance, and consumer migrations.

---

## SLO & Support Policy

- **Crate Status:** Deprecated / Legacy Support.
- **New Features:** Frozen. All UI expansion occurs in `gtk-office-suite`.
- **Maintenance Scope:** Critical security updates and migration-blocking bug fixes only.
- **Target Retirement Date:** See [ROADMAP.md](../ROADMAP.md) for the active retirement decision schedule.

---

## Operational Procedures

### 1. Verifying Crate Consumers

To check if any downstream services or repositories in Tuna OS still reference `suite-common-rust`:

```bash
# Search for git URL or crate name across repositories
grep -rn "suite-common-rust" /path/to/repos/
grep -rn "suite_common_rs" /path/to/repos/
```

If consumers are identified:
1. Verify whether they can adopt the monorepo version (`gtk-office-suite/suite-common`).
2. Update their `Cargo.toml` dependency to point to `gtk-office-suite` via git revision pin:
   ```toml
   [dependencies]
   suite-common = { git = "https://github.com/tuna-os/gtk-office-suite", rev = "<TAG_OR_COMMIT>", package = "suite-common" }
   ```

### 2. Handling Dependency & Security Alerts

1. Review proposed patch/pin updates from automated bots.
2. Confirm updates do not introduce breaking changes to existing signatures in `src/lib.rs`.
3. Test locally or via CI before accepting PRs.

### 3. Incident & Build Readiness Troubleshooting

| Issue | Root Cause | Remediation |
|---|---|---|
| Cargo build/test failure in CI | Missing GTK4 / libadwaita system headers | Ensure `gtk4-devel` and `libadwaita-devel` (or Debian equivalents `libgtk-4-dev`, `libadwaita-1-dev`) are installed in the container image. |
| Test suite headless panic/skip | No X11/Wayland display server present | GTK initialization fails headlessly. Run tests using `xvfb-run cargo test` or inside an environment with display forwarding enabled. |

---

## Escalation & Contact

- **Monorepo Target:** [tuna-os/gtk-office-suite](https://github.com/tuna-os/gtk-office-suite)
- **Tracking Issues:** [#26](https://github.com/tuna-os/suite-common-rust/issues/26), [#15](https://github.com/tuna-os/suite-common-rust/issues/15)
