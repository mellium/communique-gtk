use std::collections::HashMap;
use gtk;
use pango;

/// Fetches a translation given the language and a key representing the string.
macro_rules! translate(
    ( $key:expr ) => {
        {
            use ::res::strings::{LANG, STRINGS};
            match STRINGS.get(LANG.as_str()).unwrap_or(&STRINGS["en-us"]).get($key) {
                None => &STRINGS["en-us"][$key],
                Some(s) => *s,
            }
        }
    };

    // These two variants should only be used internally and are for defining translation maps.
    { $($key:expr => $value:expr,)+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $( m.insert($key, $value); )+
            m
        }
    };
);

lazy_static! {
    pub static ref LANG: String = gtk::get_default_language().unwrap_or(pango::language::Language::default()).to_string();
    pub static ref STRINGS: HashMap<&'static str, HashMap<&'static str, &'static str>> = translate!{
        "en-us" => translate!{
            // Low-level GTK errors
            "failed_to_start_gtk" => "Failed to initialize GTK",
            "error_registering_app"   => "Error registering application",

            // Upper
            "About"         => "About",
            "Connect"       => "Connect",
            "Conversations" => "Conversations",
            "Password"      => "Password",
            "Quit"          => "Quit",
            "Register"      => "Register",
            "Roster"        => "Roster",

            // Lower
            "user" => "user",
        },
    };
}
