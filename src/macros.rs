/// Makes it easier to clone GTK objects before moving them into closures.
/// Borrowed from the gtk-rs examples.
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

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
