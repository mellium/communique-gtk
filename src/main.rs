//! # Yokel
//!
//! A modern XMPP client in Rust and GTK3+.

#![crate_type = "bin"]
#![feature(proc_macro)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate gdk;
extern crate gdk_sys;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate gtk_sys;
extern crate regex;
extern crate toml;

use gdk::DisplayExt;
use gio::ApplicationExt;
use gtk::prelude::*;

mod config;
mod ui;
mod widget;
mod res;

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let app = gtk::Application::new(Some(res::APP_ID), gio::ApplicationFlags::FLAGS_NONE).unwrap();

    let config = config::load_config();

    match config.theme.as_ref().map(|s| s.as_ref()) {
        Some("dark") => {
            if let Some(settings) = gtk::Settings::get_default() {
                // TODO: This is deprecated, but what's the rust version of:
                // g_object_set(gtk_settings_get_default(),
                //              "gtk-application-prefer-dark-theme",
                //              TRUE,
                //              NULL);
                settings.set_property_gtk_application_prefer_dark_theme(true);
            }
        }
        Some("light") => {
            if let Some(settings) = gtk::Settings::get_default() {
                settings.set_property_gtk_application_prefer_dark_theme(false);
            }
        }
        Some("conversations") => {
            // TODO: Why doesn't this work?
            let style_provider = gtk::CssProvider::new();
            style_provider
                .load_from_data(res::STYLE_CONVERSATIONS.as_bytes())
                .unwrap();
            let display = gdk::Display::get_default().unwrap();
            let screen = display.get_default_screen();
            gtk::StyleContext::add_provider_for_screen(
                &screen,
                &style_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
        _ => {}
    }

    app.connect_startup(move |app| {
        let window = widget::AppWindow::new(&app);
        window.as_ref().connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        if config.accounts.len() == 0 {
            let login_pane = widget::Login::new();
            window.set_view(login_pane.as_ref());
        }

        app.add_window(window.as_ref());
        window.as_ref().show_all();
    });

    if let Err(e) = app.register(None) {
        eprintln!("Error registering application: {}", e);
        std::process::exit(1);
    }
    gtk::main();
}
