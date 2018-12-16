use gtk;
use gtk::EditableSignals;
use gtk::EntryExt;

use lazy_static::lazy_static;
use regex::Regex;

pub fn jid_entry<F: Fn(&str, bool) + 'static>(on_change: F) -> gtk::Entry {
    let jid_entry = gtk::Entry::new();
    jid_entry.set_placeholder_text(format!("{}@example.com", translate!("user")).as_str());
    jid_entry.set_input_purpose(gtk::InputPurpose::Email);
    jid_entry.set_activates_default(true);

    // TODO: Write a PRECIS implementation and do proper validation.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^@/]+@[^@/]+(/.+)?").unwrap();
    }

    // Setup basic (non-binding) validation of the JID entry field.
    jid_entry.connect_changed(move |entry| {
        match entry.get_text() {
            None => {
                entry.set_icon_from_icon_name(
                    gtk::EntryIconPosition::Secondary,
                    "dialog-warning-symbolic",
                );
                on_change("", false);
            }
            Some(text) => {
                if RE.is_match(&text) {
                    entry.set_icon_from_icon_name(gtk::EntryIconPosition::Secondary, None);
                    on_change(&text, true);
                } else {
                    entry.set_icon_from_icon_name(
                        gtk::EntryIconPosition::Secondary,
                        Some("dialog-warning-symbolic"),
                    );
                    on_change(&text, false);
                }
            }
        };
    });

    jid_entry
}
