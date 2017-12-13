use gtk;
use gtk::EntryExt;

pub fn pass_entry() -> gtk::Entry {
    let pass_entry = gtk::Entry::new();
    pass_entry.set_placeholder_text(translate!("Password"));
    pass_entry.set_input_purpose(gtk::InputPurpose::Password);
    pass_entry.set_activates_default(true);
    pass_entry.set_icon_from_icon_name(gtk::EntryIconPosition::Primary, "security-low-symbolic");
    pass_entry.set_icon_tooltip_text(gtk::EntryIconPosition::Primary, translate!("Show password"));
    pass_entry.set_visibility(false);
    pass_entry.connect_icon_press(|pass_entry, _, _| {
        if pass_entry.get_visibility() {
            pass_entry.set_icon_from_icon_name(gtk::EntryIconPosition::Primary, "security-low-symbolic");
            pass_entry.set_icon_tooltip_text(
                gtk::EntryIconPosition::Primary,
                translate!("Show password"),
            );
            pass_entry.set_visibility(false);
        } else {
            pass_entry.set_icon_from_icon_name(gtk::EntryIconPosition::Primary, "security-high-symbolic");
            pass_entry.set_icon_tooltip_text(
                gtk::EntryIconPosition::Primary,
                translate!("Hide password"),
            );
            pass_entry.set_visibility(true);
        }
    });
    pass_entry.set_icon_activatable(gtk::EntryIconPosition::Primary, true);

    pass_entry
}
