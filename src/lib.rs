// lib.rs — suite-common: shared GTK4/libadwaita scaffold.
// SPDX-License-Identifier: GPL-3.0-or-later

use gtk4 as gtk;
use gtk::prelude::*;

/// Create a standard GTK4 Application with the given ID.
pub fn make_app(id: &str) -> gtk::Application {
    gtk::Application::new(Some(id), Default::default())
}

/// Build a standard header bar with Open/Save buttons.
pub fn make_header_bar() -> libadwaita::HeaderBar {
    let header = libadwaita::HeaderBar::new();
    let open_btn = gtk::Button::with_label("Open");
    header.pack_start(&open_btn);
    let save_btn = gtk::Button::with_label("Save");
    header.pack_start(&save_btn);
    header
}

/// Build a standard formatting toolbar (Bold, Italic, Underline).
pub fn make_toolbar() -> gtk::Box {
    let toolbar = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    toolbar.set_halign(gtk::Align::Center);
    toolbar.add_css_class("toolbar");
    toolbar.append(&gtk::ToggleButton::with_label("B"));
    toolbar.append(&gtk::ToggleButton::with_label("I"));
    toolbar.append(&gtk::ToggleButton::with_label("U"));
    toolbar
}

/// Check if system is in dark mode.
pub fn is_dark_mode() -> bool {
    libadwaita::StyleManager::default().is_dark()
}
