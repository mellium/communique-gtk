pub const APP_NAME: &'static str = "Yokel";
pub const APP_ID: &'static str = "com.mellium.yokel";

// TODO: At some point when proc macros can fully replace normal macro_rules, we should have a
//       macro to load XML definitions at compile time and generate a procedural UI instead of
//       using a builder and finding errors at runtime.

pub const UI_MAIN_WINDOW: &'static str = include_str!("../gtk/main_window.xml");
pub const UI_LOGIN: &'static str = include_str!("../gtk/login_layout.xml");
// pub const UI_ACCOUNTS_LAYOUT: &'static str = include_str!("../ui/accounts_layout.xml");
// pub const UI_CHAT_LAYOUT: &'static str = include_str!("../ui/chat_layout.xml");

pub const STYLE_CONVERSATIONS: &'static str = include_str!("../styles/conversations.css");
