# suite-common-rust (DEPRECATED)

> ⚠️ **This crate has been superseded.**
>
> The canonical `suite-common` implementation now lives in the
> [gtk-office-suite](https://github.com/tuna-os/gtk-office-suite) monorepo
> (`gtk-office-suite/suite-common/`).
>
> This standalone crate is an early extraction that contains only the original
> small set of GTK and libadwaita helpers. The actively maintained monorepo
> version has a substantially expanded API and includes `suite-common-core`.
>
> **Do not add new dependencies on this crate.** Use the monorepo version
> instead, or extract it from there if standalone publishing is desired.
>
> This repository is retained for historical reference only.

## Migration

```toml
# Instead of this crate, use:
[dependencies]
suite-common = { git = "https://github.com/tuna-os/gtk-office-suite", package = "suite-common" }
suite-common-core = { git = "https://github.com/tuna-os/gtk-office-suite", package = "suite-common-core" }
```
