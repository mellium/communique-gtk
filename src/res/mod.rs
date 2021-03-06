pub mod strings;

pub const APP_NAME: &str = "Communiqué";
pub const APP_ID: &str = "im.mellium.yokel";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// NIST SP 800-63B <https://pages.nist.gov/800-63-3/sp800-63b.html> recommends 8 characters as a
// minimum length requirement.
// Add a margin of safety for the suggested length.
pub const SUGGESTED_PASSWORD_LEN: f64 = 16.0;

pub const SVG_LOGO: &[u8] = include_bytes!("../../res/img/logo_concept_4_plain.svg");

pub const STYLE_AVATAR: &[u8] = include_bytes!("../../res/style/avatar.css");
pub const STYLE_CONVERSATIONS: &[u8] = include_bytes!("../../res/style/conversations.css");
pub const STYLE_LIST: &[u8] = include_bytes!("../../res/style/list.css");
pub const STYLE_MAIN: &[u8] = include_bytes!("../../res/style/main.css");
