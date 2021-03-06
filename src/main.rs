//! A instant messaging client in Rust and GTK3+.
//!
//! Communiqué works with instant messaging services that support the Extensible Messaging and
//! Presence Protocol (XMPP), historically known as "Jabber".
//!
//! ## Developers
//!
//! ### GIO Actions
//!
//! The following GIO actions are defined:
//!
//! - `app.about` — launches the about dialog
//! - `app.close` — closes the application
//! - `app.login` — login all active but offline accounts
//! - `win.search` — toggle the search bar

#![crate_type = "bin"]
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico", html_root_url = "/")]

#[macro_use]
mod macros;

mod app;
mod config;
mod res;
mod ui;
mod widget;

use std::env::args;
use std::process;

// This is the main entrypoint for the program. It should handle anything that needs to happen
// before GTK takes over, for instance, OS signals, dealing with command line args, and exiting the
// process on errors (nothing else should ever call process::exit).
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
