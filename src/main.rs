//! # Yokel
//!
//! A modern XMPP client in Rust and GTK3+.

#![crate_type = "bin"]

#[macro_use]
extern crate serde_derive;

extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
extern crate gio;
extern crate toml;

mod res;
mod config;


fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let builder = gtk::Builder::new_from_string(res::UI_MAIN_WINDOW);

    let config = config::load_config();
    if config.accounts.len() == 0 {
        let mainbox = builder.get_object::<gtk::Box>("main_view_box").unwrap();
        let login_builder = gtk::Builder::new_from_string(res::UI_LOGIN);
        if let Some(login_box) = login_builder.get_object::<gtk::Box>("login_box") {
            mainbox.add(&login_box);
        }
    }

    // TODO: Why doesn't this work?
    if config.theme != "system_dark" && config.theme != "system_light" {
        let style_provider = gtk::CssProvider::new();
        style_provider.load_from_data(res::STYLE_CONVERSATIONS).unwrap();
        let display = gdk::Display::get_default().unwrap();
        let screen = display.get_default_screen();
        gtk::StyleContext::add_provider_for_screen(&screen,
                                                   &style_provider,
                                                   gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let app = gtk::Application::new(Some(res::APP_ID), gio::APPLICATION_FLAGS_NONE).unwrap();

    // TODO: app.register and then move things to the startup handler.
    // app.connect_startup(move |app| {
    let window = builder.get_object::<gtk::ApplicationWindow>("main_window").unwrap();
    window.set_title(res::APP_NAME);
    window.set_default_size(350, 70);
    window.set_position(gtk::WindowPosition::Center);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    app.add_window(&window);

    // TODO: Move this into startup event when register() is supported by gtk-rs.
    window.show_all();
    gtk::main();
}
