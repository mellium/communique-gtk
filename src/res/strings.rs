use std::collections::HashMap;
use gtk;

lazy_static! {
    pub static ref LANG: String = gtk::get_default_language().unwrap_or_default().to_string();
    pub static ref STRINGS: HashMap<&'static str, HashMap<&'static str, &'static str>> = translate!{
        "en-us" => translate!{
            // Low-level GTK errors
            "failed_to_start_gtk" => "Failed to initialize GTK",
            "error_registering_app"   => "Error registering application",

            // Upper
            "About"         => "About",
            "Close"         => "Close",
            "Connect"       => "Connect",
            "Conversations" => "Conversations",
            "Password"      => "Password",
            "Register"      => "Register",
            "Roster"        => "Roster",

            // Lower
            "user" => "user",
        },
    };
}
