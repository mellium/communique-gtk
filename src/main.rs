//! # Yokel
//!
//! A modern XMPP client in Rust and GTK3+.

#![crate_type = "bin"]

mod res;

extern crate gdk;
extern crate gio;
extern crate serde;
extern crate toml;

extern crate gtk;
use gtk::prelude::*;

include!(concat!(env!("OUT_DIR"), "/config.rs"));

fn main() {

    gtk::init().expect("Failed to initialize GTK");

    let builder = gtk::Builder::new_from_string(res::UI_MAIN_WINDOW);

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

    let config = load_config();
    if config.accounts.len() == 0 {
        if let Some(mainbox) = builder.get_object::<gtk::Box>("main_view_box") {
            let login_builder = gtk::Builder::new_from_string(res::UI_LOGIN);
            if let Some(login_box) = login_builder.get_object::<gtk::Box>("login_box") {
                mainbox.add(&login_box);
            }
        }
    }

    // TODO: Move this into startup event when register() is supported by gtk-rs.
    window.show_all();
    gtk::main();
}
