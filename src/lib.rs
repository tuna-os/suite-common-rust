// lib.rs — suite-common: shared GTK4/libadwaita scaffold.
// SPDX-License-Identifier: GPL-3.0-or-later

use libadwaita as adw;
use gtk4::prelude::*;

/// Create a standard GTK4 Application with the given ID.
pub fn make_app(id: &str) -> gtk4::Application {
    gtk4::Application::new(Some(id))
}

/// Build a standard header bar with Open/Save buttons.
pub fn make_header_bar() -> adw::HeaderBar {
    let header = adw::HeaderBar::new();
    header.pack_start(&gtk4::Button::with_label("Open"));
    header.pack_end(&gtk4::Button::with_label("Save"));
    header
}

/// Build a centered formatting toolbar.
pub fn make_toolbar() -> gtk4::Box {
    let toolbar = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    toolbar.set_halign(gtk4::Align::Center);
    toolbar.add_css_class("toolbar");
    for label in &["B", "I", "U"] {
        toolbar.append(&gtk4::ToggleButton::with_label(label));
    }
    toolbar
}

/// Check if system prefers dark color scheme.
pub fn is_dark_mode() -> bool {
    adw::StyleManager::default().is_dark()
}
