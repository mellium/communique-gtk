use gdk_pixbuf;

use gtk;
use gtk::BoxExt;
use gtk::ContainerExt;
use gtk::ImageExt;
use gtk::SpinnerExt;
use gtk::StackExt;
use gtk::WidgetExt;

/// A loading view that is shown when accounts are logging in.
pub struct Spinner {
    stack: gtk::Stack,
}

impl Spinner {
    /// Creates a new loading spinner.
    pub fn new(logobuf: &gdk_pixbuf::Pixbuf) -> Spinner {
        let stack = gtk::Stack::new();

        let center_box = gtk::Box::new(gtk::Orientation::Vertical, 15);
        center_box.set_can_focus(false);
        center_box.set_halign(gtk::Align::Center);
        center_box.set_valign(gtk::Align::Center);
        center_box.set_spacing(15);
        stack.add_named(&center_box, translate!("Loading"));

        let logo = gtk::Image::new_from_pixbuf(logobuf);
        logo.set_can_focus(false);
        logo.set_margin_bottom(25);
        logo.set_pixel_size(100);
        center_box.add(&logo);

        let spinner = gtk::Spinner::new();
        spinner.start();
        center_box.add(&spinner);

        Spinner { stack: stack }
    }
}

impl AsRef<gtk::Stack> for Spinner {
    #[inline]
    fn as_ref(&self) -> &gtk::Stack {
        &self.stack
    }
}
