use res;
use gdk_pixbuf;
use gdk;

use gtk;
use gtk::AboutDialogExt;
use gtk::GtkWindowExt;

pub fn about_dialog<'a, P: gtk::IsA<gtk::Window> + 'a, Q: Into<Option<&'a P>>>(
    parent: Q,
    logobuf: &gdk_pixbuf::Pixbuf,
) -> gtk::AboutDialog {
    let p = gtk::AboutDialog::new();
    p.set_authors(&["Sam Whited"]);
    p.set_copyright("Copyright © 2017 The Communiqué Authors.\nAll rights reserved.");
    p.set_destroy_with_parent(true);
    p.set_license_type(gtk::License::Bsd);
    p.set_logo(logobuf);
    p.set_program_name(res::APP_NAME);
    p.set_skip_pager_hint(true);
    p.set_skip_taskbar_hint(true);
    p.set_title(translate!("About"));
    p.set_transient_for(parent);
    p.set_type_hint(gdk::WindowTypeHint::Splashscreen);
    p.set_version(res::VERSION);
    p.set_website("https://mellium.im");
    p.set_website_label("mellium.im");
    p.add_credit_section(
        translate!("Open Source"),
        &[
            "Gtk-rs http://gtk-rs.org/ (MIT)",
            "Material Design icons https://material.io/icons/ (Apache-2.0)",
            "Rust team libs https://www.rust-lang.org/ (MIT/Apache-2.0)",
            "toml-rs https://crates.io/crates/toml (MIT/Apache-2.0)",
        ],
    );

    p
}
