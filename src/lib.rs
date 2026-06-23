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
