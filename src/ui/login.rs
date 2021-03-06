use gdk_pixbuf;
use gtk;
use gtk::ActionableExt;
use gtk::BoxExt;
use gtk::ContainerExt;
use gtk::EditableSignals;
use gtk::EntryExt;
use gtk::ImageExt;
use gtk::StyleContextExt;
use gtk::WidgetExt;

use crate::res;
use crate::widget;

/// The Login widget provides a username and password text entry as well as register and login
/// buttons.
pub struct Login {
    view: gtk::Box,
    def: gtk::Button,
}

impl Login {
    /// Creates a new login pane that can be shown in the application.
    pub fn new(logobuf: &gdk_pixbuf::Pixbuf) -> Login {
        let center_box = gtk::Box::new(gtk::Orientation::Vertical, 15);
        center_box.set_can_focus(false);
        center_box.set_halign(gtk::Align::Center);
        center_box.set_valign(gtk::Align::Center);
        center_box.set_spacing(15);

        let logo = gtk::Image::new_from_pixbuf(logobuf);
        logo.set_can_focus(false);
        logo.set_margin_bottom(25);
        logo.set_pixel_size(100);
        center_box.add(&logo);

        let entry_box = gtk::Box::new(gtk::Orientation::Vertical, 15);
        entry_box.set_can_focus(false);
        center_box.add(&entry_box);

        let connect = gtk::Button::new_with_label(translate!("Connect"));
        connect.set_action_name("app.login");
        connect.set_can_default(true);
        if let Some(c) = connect.get_style_context() {
            c.add_class("suggested-action");
        }

        let entry_conn_button = connect.clone();
        let jid_entry = widget::jid_entry(move |_, ok| entry_conn_button.set_sensitive(ok));
        entry_box.add(&jid_entry);

        let pass_entry = widget::pass_entry();
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

        let register = gtk::Button::new_with_label(translate!("Register"));
        register.set_sensitive(false);
        button_box.add(&register);
        button_box.add(&connect);
        button_box.set_child_packing(&register, false, false, 0, gtk::PackType::End);
        button_box.set_child_packing(&connect, false, false, 0, gtk::PackType::End);

        Login {
            view: center_box,
            def: connect,
        }
    }

    /// Causes self to become the default widget.
    pub fn grab_default(&self) {
        self.def.grab_default();
    }
}

impl AsRef<gtk::Box> for Login {
    #[inline]
    fn as_ref(&self) -> &gtk::Box {
        &self.view
    }
}
