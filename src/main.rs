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
use pane::Pane;

mod config;
mod login;
mod menu;
mod pane;
mod res;

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let builder = gtk::Builder::new_from_string(res::UI_MAIN_WINDOW);
    menu::junk_drawer(&builder);

    let config = config::load_config();
    if config.accounts.len() == 0 {
        let mainbox = builder.get_object::<gtk::Box>("main_view_box").unwrap();
        let login_pane = login::Login::<gtk::Box>::new().unwrap();
        mainbox.add(&login_pane.get_widget());
    }

    let app = gtk::Application::new(Some(res::APP_ID), gio::ApplicationFlags::FLAGS_NONE).unwrap();

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

    app.connect_startup(move |a| {
        let window = builder
            .get_object::<gtk::ApplicationWindow>("main_window")
            .expect("Failed to construct the main window");
        window.set_title(res::APP_NAME);
        window.set_default_size(350, 70);
        window.set_position(gtk::WindowPosition::Center);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        a.add_window(&window);
        window.show_all();
    });

    match app.register(None) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error registering application: {}", e);
            std::process::exit(1);
        }
    }
    gtk::main();
}
