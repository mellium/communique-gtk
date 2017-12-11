use gtk;
use gtk::BoxExt;
use gtk::ContainerExt;
use gtk::EditableSignals;
use gtk::EntryExt;
use gtk::FrameExt;
use gtk::ImageExt;
use gtk::StackExt;
use gtk::StyleContextExt;
use gtk::WidgetExt;

use gdk_pixbuf;
use regex::Regex;
use res;

/// The Login widget provides a username and password text entry as well as register and login
/// buttons.
pub struct Login {
    stack: gtk::Stack,
    def: gtk::Button,
}

impl Login {
    /// Creates a new login pane that can be shown in the application.
    pub fn new(logobuf: &gdk_pixbuf::Pixbuf) -> Login {
        let stack = gtk::Stack::new();

        let frame = gtk::Frame::new(None);
        frame.set_can_focus(false);
        frame.set_shadow_type(gtk::ShadowType::None);
        let tab = translate!("Login");
        stack.add_titled(&frame, tab, tab);

        let center_box = gtk::Box::new(gtk::Orientation::Vertical, 15);
        center_box.set_can_focus(false);
        center_box.set_halign(gtk::Align::Center);
        center_box.set_valign(gtk::Align::Center);
        center_box.set_spacing(15);
        frame.add(&center_box);

        let logo = gtk::Image::new_from_pixbuf(logobuf);
        logo.set_can_focus(false);
        logo.set_margin_bottom(25);
        logo.set_pixel_size(100);
        center_box.add(&logo);

        let entry_box = gtk::Box::new(gtk::Orientation::Vertical, 15);
        entry_box.set_can_focus(false);
        center_box.add(&entry_box);

        let jid_entry = gtk::Entry::new();
        jid_entry.set_placeholder_text(format!("{}@example.com", translate!("user")).as_str());
        jid_entry.set_input_purpose(gtk::InputPurpose::Email);
        jid_entry.set_activates_default(true);
        entry_box.add(&jid_entry);

        // TODO: Write a PRECIS implementation and do proper validation.
        //       This is just to test the input field really.
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[^@/]+@[^@/]+(/.+)?").unwrap();
        }

        // Setup basic (non-binding) validation of the JID entry field.
        jid_entry.connect_changed(|entry| match entry.get_text() {
            None => {
                entry.set_icon_from_icon_name(
                    gtk::EntryIconPosition::Secondary,
                    Some("dialog-warning-symbolic"),
                );
            }
            Some(text) => {
                if RE.is_match(&text) {
                    entry.set_icon_from_icon_name(gtk::EntryIconPosition::Secondary, None);
                } else {
                    entry.set_icon_from_icon_name(
                        gtk::EntryIconPosition::Secondary,
                        Some("dialog-warning-symbolic"),
                    );
                }
            }
        });

        let pass_entry = gtk::Entry::new();
        pass_entry.set_placeholder_text(translate!("Password"));
        pass_entry.set_input_purpose(gtk::InputPurpose::Password);
        pass_entry.set_visibility(false);
        pass_entry.set_activates_default(true);
        entry_box.add(&pass_entry);
        // Perform some simple length checks on the password entry field.
        if res::SUGGESTED_PASSWORD_LEN > 0.0 {
            pass_entry.connect_changed(|entry| {
                entry.set_progress_fraction(
                    f64::from(entry.get_text_length()) / res::SUGGESTED_PASSWORD_LEN,
                )
            });
        }

        let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 15);
        button_box.set_can_focus(false);
        center_box.add(&button_box);

        let connect = gtk::Button::new_with_label(translate!("Connect"));
        connect.set_can_default(true);
        match connect.get_style_context() {
            None => {}
            Some(c) => c.add_class("suggested-action"),
        }

        let register = gtk::Button::new_with_label(translate!("Register"));
        register.set_sensitive(false);
        button_box.add(&register);
        button_box.add(&connect);
        button_box.set_child_packing(&register, false, false, 0, gtk::PackType::End);
        button_box.set_child_packing(&connect, false, false, 0, gtk::PackType::End);

        Login {
            stack: stack,
            def: connect,
        }
    }

    /// Causes self to become the default widget.
    pub fn grab_default(&self) {
        self.def.grab_default();
    }
}

impl AsRef<gtk::Stack> for Login {
    #[inline]
    fn as_ref(&self) -> &gtk::Stack {
        &self.stack
    }
}
