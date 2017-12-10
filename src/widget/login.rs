use gtk;
use gtk::BoxExt;
use gtk::ContainerExt;
use gtk::EditableSignals;
use gtk::EntryExt;
use gtk::FrameExt;
use gtk::ImageExt;
use gtk::WidgetExt;

use gdk_pixbuf;
use glib::Cast;
use regex::Regex;
use res;

/// The Login widget provides a username and password text entry as well as register and login
/// buttons.
pub struct Login {
    view: gtk::Widget,
}

impl Login {
    pub fn new(logobuf: &gdk_pixbuf::Pixbuf) -> Login {
        let frame = gtk::Frame::new(None);
        frame.set_can_focus(false);
        frame.set_shadow_type(gtk::ShadowType::None);

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
        entry_box.add(&pass_entry);
        // Perform some simple length checks on the password entry field.
        if res::SUGGESTED_PASSWORD_LEN > 0.0 {
            pass_entry.connect_changed(|entry| match entry.get_text_length() {
                0 => entry.set_progress_fraction(0.0),
                l @ _ => entry.set_progress_fraction((l + 1) as f64 / res::SUGGESTED_PASSWORD_LEN),
            });
        }

        let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 15);
        button_box.set_can_focus(false);
        center_box.add(&button_box);

        let connect = gtk::Button::new_with_label(translate!("Connect"));
        let register = gtk::Button::new_with_label(translate!("Register"));
        button_box.add(&register);
        button_box.add(&connect);
        button_box.set_child_packing(&register, false, false, 0, gtk::PackType::End);
        button_box.set_child_packing(&connect, false, false, 0, gtk::PackType::End);

        return Login { view: frame.upcast::<gtk::Widget>() };
    }
}

impl AsRef<gtk::Widget> for Login {
    #[inline]
    fn as_ref(&self) -> &gtk::Widget {
        return &self.view;
    }
}
