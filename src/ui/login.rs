use gtk;
use gtk::EditableSignals;
use gtk::EntryExt;
use regex::Regex;
use res;
use ui::pane;

pub struct Login<T: gtk::IsA<gtk::Widget>> {
    builder: gtk::Builder,
    view: T,
}

impl<U: gtk::IsA<gtk::Widget> + gtk::IsA<gtk::Object>> Login<U> {
    pub fn new() -> Option<Login<U>> {
        // TODO: Write a PRECIS implementation and do proper validation.
        //       This is just to test the input field really.
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[^@/]+@[^@/]+(/.+)?").unwrap();
        }
        let builder = gtk::Builder::new_from_string(res::UI_LOGIN);

        // Setup basic (non-binding) validation of the JID entry field.
        if let Some(jid_entry) = builder.get_object::<gtk::Entry>("jid") {
            jid_entry.connect_insert_text(|entry, new_text, _| {
                let text = entry.get_buffer().get_text();
                let new_entry = text + new_text;
                if RE.is_match(&new_entry) {
                    entry.set_icon_from_icon_name(gtk::EntryIconPosition::Secondary, None);
                } else {
                    entry.set_icon_from_icon_name(
                        gtk::EntryIconPosition::Secondary,
                        Some("dialog-warning-symbolic"),
                    );
                }
            });
            jid_entry.connect_delete_text(|entry, _, _| {
                let text = entry.get_buffer().get_text();
                if RE.is_match(&text) {
                    entry.set_icon_from_icon_name(gtk::EntryIconPosition::Secondary, None);
                } else {
                    entry.set_icon_from_icon_name(
                        gtk::EntryIconPosition::Secondary,
                        Some("dialog-warning-symbolic"),
                    );
                }
            });
        }
        // Perform some simple length checks on the password entry field.
        if let Some(password_entry) = builder.get_object::<gtk::Entry>("password") {
            if res::SUGGESTED_PASSWORD_LEN > 0.0 {
                password_entry.connect_insert_text(|entry, _, _| {
                    entry.set_progress_fraction(
                        (entry.get_text_length() + 1) as f64 / res::SUGGESTED_PASSWORD_LEN,
                    );
                });
                password_entry.connect_delete_text(|entry, start, end| {
                    let len = entry.get_text_length();
                    if end < 0 {
                        entry.set_progress_fraction(0.0);
                    } else {
                        entry.set_progress_fraction(
                            (len - (end - start) as u16) as f64 / res::SUGGESTED_PASSWORD_LEN,
                        );
                    }
                });
            }
        }
        match builder.get_object::<U>("login_box") {
            Some(view) => {
                Some(Login::<U> {
                    builder: builder,
                    view: view,
                })
            }
            None => None,
        }
    }
}

impl<U: gtk::IsA<gtk::Widget>> pane::Pane<U> for Login<U> {
    fn get_widget(self) -> U {
        self.view
    }
    fn get_builder(self) -> gtk::Builder {
        self.builder
    }
}
