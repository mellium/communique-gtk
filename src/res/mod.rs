pub mod strings;

pub const APP_NAME: &'static str = "Communiqué";
pub const APP_ID: &'static str = "im.mellium.yokel";

// NIST SP 800-63B <https://pages.nist.gov/800-63-3/sp800-63b.html> recommends 8 characters as a
// minimum length requirement.
// Add a margin of safety for the suggested length.
pub const SUGGESTED_PASSWORD_LEN: f64 = 16.0;

pub const SVG_LOGO: &'static [u8] = include_bytes!("../../img/logo_concept_4_plain.svg");

pub const STYLE_MAIN: &'static [u8] = include_bytes!("../../style/main.css");
pub const STYLE_CONVERSATIONS: &'static [u8] = include_bytes!("../../style/conversations.css");
