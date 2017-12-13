pub mod strings;

pub const APP_NAME: &str = "Communiqué";
pub const APP_ID: &str = "im.mellium.yokel";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// NIST SP 800-63B <https://pages.nist.gov/800-63-3/sp800-63b.html> recommends 8 characters as a
// minimum length requirement.
// Add a margin of safety for the suggested length.
pub const SUGGESTED_PASSWORD_LEN: f64 = 16.0;

pub const SVG_LOGO: &[u8] = include_bytes!("../../img/logo_concept_4_plain.svg");

pub const STYLE_MAIN: &[u8] = include_bytes!("../../style/main.css");
pub const STYLE_CONVERSATIONS: &[u8] = include_bytes!("../../style/conversations.css");

pub const ICON_VISIBILITY: &[u8] = include_bytes!("../../img/ic_visibility_black_48px.svg");
pub const ICON_VISIBILITY_OFF: &[u8] = include_bytes!("../../img/ic_visibility_off_black_48px.svg");
pub const ICON_VISIBILITY_LIGHT: &[u8] = include_bytes!("../../img/ic_visibility_white_48px.svg");
pub const ICON_VISIBILITY_OFF_LIGHT: &[u8] =
    include_bytes!("../../img/ic_visibility_off_white_48px.svg");
