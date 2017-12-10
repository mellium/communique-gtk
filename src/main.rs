//! # Communiqué
//!
//! A instant messaging client in Rust and GTK3+.
//!
//! Communiqué works with instant messaging services that support the Extensible Messaging and
//! Presence Protocol (XMPP), historically known as "Jabber".

#![crate_type = "bin"]
#![feature(proc_macro)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate gdk;
extern crate gdk_pixbuf;
extern crate gdk_sys;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate gtk_sys;
extern crate pango;
extern crate regex;
extern crate toml;

use std::env::args;

use gdk::DisplayExt;
use gio::ApplicationExt;
use gio::ApplicationExtManual;
use gtk::CssProviderExt;
use gtk::GtkApplicationExt;
use gtk::SettingsExt;
use gtk::WidgetExt;

#[macro_use]
mod macros;

mod config;
mod res;
mod ui;
mod widget;

fn main() {
    gtk::init().expect(translate!("failed_to_start_gtk"));
    let app = gtk::Application::new(Some(res::APP_ID), gio::ApplicationFlags::FLAGS_NONE).unwrap();

    let config = config::load_config();

    app.connect_startup(|_| {});
    app.connect_activate(move |app| {
        let display = gdk::Display::get_default().unwrap();
        let screen = display.get_default_screen();
        let style_provider = gtk::CssProvider::new();
        style_provider.load_from_data(res::STYLE_MAIN).unwrap();

        match config.theme.as_ref().map(|s| s.as_ref()) {
            Some("dark") => {
                if let Some(settings) = gtk::Settings::get_default() {
                    settings.set_property_gtk_application_prefer_dark_theme(true);
                }
            }
            Some("light") => {
                if let Some(settings) = gtk::Settings::get_default() {
                    settings.set_property_gtk_application_prefer_dark_theme(false);
                }
            }
            Some("conversations") => {
                style_provider
                    .load_from_data(res::STYLE_CONVERSATIONS)
                    .unwrap();
                gtk::StyleContext::add_provider_for_screen(
                    &screen,
                    &style_provider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }
            _ => {}
        }

        let logoloader = gdk_pixbuf::PixbufLoader::new();
        logoloader.loader_write(res::SVG_LOGO).unwrap();
        logoloader.close().unwrap();
        let logobuf = logoloader.get_pixbuf().unwrap();

        let window = widget::AppWindow::new(&app, &logobuf);
        if config.accounts.len() == 0 {
            let login_pane = widget::Login::new(&logobuf);
            window.set_view(login_pane.as_ref());
            login_pane.grab_default();
        }

        app.add_window(window.as_ref());
        window.as_ref().show_all();
    });

    if let Err(e) = app.register(None) {
        eprintln!("{}: {}", translate!("error_registering_app"), e);
        std::process::exit(1);
    }
    app.run(&args().collect::<Vec<_>>());
}
