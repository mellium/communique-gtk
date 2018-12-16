use gtk;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LANG: String = gtk::get_default_language().unwrap_or_default().to_string();
    pub static ref STRINGS: HashMap<&'static str, HashMap<&'static str, &'static str>> = translate!{
        "en-us" => translate!{
            // Low-level GTK errors
            "failed_to_start_gtk"   => "Failed to initialize GTK",
            "error_registering_app" => "Error registering application",

            // Upper
            "About"         => "About",
            "Close"         => "Close",
            "Conferences"   => "Conferences",
            "Connect"       => "Connect",
            "Conversations" => "Conversations",
            "Login"         => "Login",
            "Open Source"   => "Open Source",
            "Password"      => "Password",
            "Register"      => "Register",
            "Roster"        => "Roster",

            // Lower
            "user" => "user",

            // Mixed
            "Hide password" => "Hide password",
            "Show password" => "Show password",
        },
    };
}
