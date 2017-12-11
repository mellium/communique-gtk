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
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate pango;
extern crate regex;
extern crate toml;

#[macro_use]
mod macros;

mod app;
mod config;
mod res;
mod ui;

use std::process;
use std::env::args;

// main is the main entrypoint for the program. It should handle anything that needs to happen
// before GTK takes over, for instance, handling OS signals, dealing with command line args, and
// exiting the process on errors (nothing else should ever call process::exit);
fn main() {
    gtk::init().expect(translate!("failed_to_start_gtk"));
    match app::App::new() {
        Err(e) => {
            eprintln!("{}: {:?}", translate!("error_registering_app"), e);
            process::exit(1);
        }
        Ok(app) => {
            app.run(&args().collect::<Vec<_>>());
        }
    }
}
