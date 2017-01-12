//! # Yokel
//!
//! A modern XMPP client in Rust and GTK3+.

#![crate_type = "bin"]

mod res;

extern crate gdk;
use gdk::RGBA;

extern crate gtk;
use gtk::prelude::*;

extern crate serde;
extern crate toml;

include!(concat!(env!("OUT_DIR"), "/config.rs"));

fn main() {

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let config = load_config();
    match &config.theme as &str {
        // TODO: Why aren't themes working?
        "conversations" => {
            let style_provider = gtk::CssProvider::new();
            style_provider.load_from_data(res::STYLE_CONVERSATIONS).unwrap();
            if let Some(display) = gdk::Display::get_default() {
                let screen = display.get_default_screen();
                gtk::StyleContext::add_provider_for_screen(&screen,
                                                       &style_provider,
                                                       gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
            }
        }
        _ => {}
    }

    let builder = gtk::Builder::new_from_string(res::UI_MAIN_WINDOW);

    if let Some(mainbox) = builder.get_object::<gtk::Box>("main_view_box") {
        // If accounts == 0
        let login = gtk::Builder::new_from_string(res::UI_LOGIN);

        if let Some(login_box) = login.get_object::<gtk::Box>("login_box") {
            mainbox.add(&login_box);
        }
    }

    if let Some(window) = builder.get_object::<gtk::Window>("main_window") {
        let header_bar = builder.get_object::<gtk::HeaderBar>("header_bar");
        window.set_titlebar(header_bar.as_ref());
        if let Some(header_bar) = header_bar {

            // This causes all the buttons to show on i3, which is probably not what we want.
            // Is there a way to always show the close button, but hint that we want the defaults
            // for the other buttons (which I assume is "hidden" on i3)?
            // Is that even how window managers work?
            // header_bar.set_show_close_button(true);
            header_bar.override_color(gtk::STATE_FLAG_NORMAL | gtk::STATE_FLAG_ACTIVE,
                                      &RGBA {
                                          red: 0.0,
                                          green: 0.0,
                                          blue: 1.0,
                                          alpha: 1.0,
                                      });
        }

        window.set_title(res::APP_NAME);
        window.set_default_size(350, 70);
        window.set_position(gtk::WindowPosition::Center);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.show_all();
    }

    gtk::main();
}
