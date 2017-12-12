use gdk_pixbuf;
use res;

use gtk;
use gtk::EntryExt;

pub fn pass_entry() -> gtk::Entry {
    let iconloader = gdk_pixbuf::PixbufLoader::new();
    iconloader.loader_write(res::ICON_VISIBILITY).unwrap();
    iconloader.close().unwrap();
    let visibility = iconloader.get_pixbuf().unwrap();

    let iconloader = gdk_pixbuf::PixbufLoader::new();
    iconloader.loader_write(res::ICON_VISIBILITY_OFF).unwrap();
    iconloader.close().unwrap();
    let visibility_off = iconloader.get_pixbuf().unwrap();

    let pass_entry = gtk::Entry::new();
    pass_entry.set_placeholder_text(translate!("Password"));
    pass_entry.set_input_purpose(gtk::InputPurpose::Password);
    pass_entry.set_visibility(false);
    pass_entry.set_activates_default(true);
    pass_entry.set_icon_from_pixbuf(gtk::EntryIconPosition::Primary, &visibility);
    pass_entry.set_icon_tooltip_text(gtk::EntryIconPosition::Primary, translate!("Show password"));
    pass_entry.connect_icon_press(
        clone!( pass_entry, visibility, visibility_off => move |_, _, _| {
        let v = pass_entry.get_icon_tooltip_text(gtk::EntryIconPosition::Primary);
        if v.is_some() && v.unwrap() == translate!("Show password") {
            pass_entry.set_icon_from_pixbuf(
                gtk::EntryIconPosition::Primary,
                &visibility_off,
                );
            pass_entry.set_icon_tooltip_text(
                gtk::EntryIconPosition::Primary,
                translate!("Hide password"),
                );
            pass_entry.set_visibility(true);
        } else {
            pass_entry.set_icon_from_pixbuf(
                gtk::EntryIconPosition::Primary,
                &visibility,
                );
            pass_entry.set_icon_tooltip_text(
                gtk::EntryIconPosition::Primary,
                translate!("Show password"),
                );
            pass_entry.set_visibility(false);
        }
    }),
    );
    pass_entry.set_icon_activatable(gtk::EntryIconPosition::Primary, true);

    pass_entry
}
