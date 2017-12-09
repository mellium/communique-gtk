pub const APP_NAME: &'static str = "Communiqué";
pub const APP_ID: &'static str = "com.mellium.yokel";

// NIST SP 800-63B <https://pages.nist.gov/800-63-3/sp800-63b.html> recommends 8 passwords as a
// minimum length requirement.
// Add a factor of safety for the suggested length.
pub const SUGGESTED_PASSWORD_LEN: f64 = 16.0;

pub const UI_MAIN_WINDOW: &'static str = include_str!("../gtk/main_window.xml");

pub const STYLE_CONVERSATIONS: &'static str = include_str!("../styles/conversations.css");
