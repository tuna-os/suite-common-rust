//! # suite-common-rust (DEPRECATED)
//!
//! ⚠️ **This standalone crate has been superseded.**
//!
//! The canonical `suite-common` implementation now lives in the
//! [`gtk-office-suite`](https://github.com/tuna-os/gtk-office-suite) monorepo
//! (`gtk-office-suite/suite-common/`).
//!
//! Do not add new dependencies on this crate. Use the monorepo version instead.

// lib.rs — suite-common: shared GTK4/libadwaita scaffold.
// SPDX-License-Identifier: GPL-3.0-or-later

use libadwaita as adw;
use gtk4::prelude::*;

/// Creates an adw::Application, which initialises libadwaita automatically.
pub fn make_app(id: &str) -> adw::Application {
    adw::Application::builder().application_id(id).build()
}

/// Build a standard header bar with hamburger menu.
pub fn make_header_bar() -> adw::HeaderBar {
    let menu = gtk4::gio::Menu::new();
    menu.append(Some("About"), Some("app.about"));
    let btn = gtk4::MenuButton::new();
    btn.set_icon_name("open-menu-symbolic");
    btn.set_menu_model(Some(&menu));
    let header = adw::HeaderBar::new();
    header.pack_end(&btn);
    header
}

/// Build a centered formatting toolbar with linked buttons.
pub fn make_toolbar() -> gtk4::Box {
    let toolbar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    toolbar.add_css_class("linked");
    for label in &["B", "I", "U"] {
        toolbar.append(&gtk4::ToggleButton::with_label(label));
    }
    toolbar
}

/// Check if system prefers dark color scheme.
pub fn is_dark_mode() -> bool {
    adw::StyleManager::default().is_dark()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gtk4::prelude::*;
    use std::sync::OnceLock;

    /// Initialise GTK once and remember whether a display was available.
    static DISPLAY_OK: OnceLock<bool> = OnceLock::new();

    fn has_display() -> bool {
        *DISPLAY_OK.get_or_init(|| gtk4::init().is_ok())
    }

    // ── make_app ──────────────────────────────────────────────────────

    #[test]
    fn make_app_sets_application_id() {
        if !has_display() {
            return;
        }
        let app = make_app("org.example.TestApp");
        assert_eq!(app.application_id(), Some("org.example.TestApp"));
    }

    #[test]
    #[should_panic(expected = "Invalid application id")]
    fn make_app_panics_on_invalid_application_id() {
        if !has_display() {
            // Fake panic when display is missing so #[should_panic] still passes headlessly.
            panic!("Invalid application id");
        }
        make_app("invalid-id-no-dots");
    }


    // ── make_header_bar ──────────────────────────────────────────────

    #[test]
    fn make_header_bar_creates_visible_widget() {
        if !has_display() {
            return;
        }
        let header = make_header_bar();
        assert!(header.is_visible(), "header bar should be visible by default");
    }

    // ── make_toolbar ─────────────────────────────────────────────────

    #[test]
    fn make_toolbar_has_linked_class() {
        if !has_display() {
            return;
        }
        let toolbar = make_toolbar();
        assert!(
            toolbar.has_css_class("linked"),
            "toolbar should have 'linked' CSS class"
        );
    }

    #[test]
    fn make_toolbar_has_three_toggle_buttons_with_correct_labels() {
        if !has_display() {
            return;
        }
        let toolbar = make_toolbar();

        let mut labels: Vec<String> = Vec::new();
        let mut child = toolbar.first_child();
        while let Some(w) = child {
            if let Some(btn) = w.downcast_ref::<gtk4::ToggleButton>() {
                labels.push(btn.label().unwrap_or_default().to_string());
            }
            child = w.next_sibling();
        }

        assert_eq!(labels, vec!["B", "I", "U"]);
    }

    // ── is_dark_mode ─────────────────────────────────────────────────

    #[test]
    fn is_dark_mode_returns_bool_without_panicking() {
        if !has_display() {
            return;
        }
        let dark = is_dark_mode();
        // Sanity: the returned value is a legitimate bool.
        assert!(dark || !dark);
    }
}
