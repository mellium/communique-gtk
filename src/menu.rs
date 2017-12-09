use gtk;
use gtk::MenuShellExt;
use gtk::WidgetExt;
use gtk::MenuButtonExt;
use gtk::MenuExt;

pub fn junk_drawer(builder: &gtk::Builder) {
    let junk_drawer: gtk::MenuButton = builder
        .get_object::<gtk::MenuButton>("junk_drawer")
        .expect("Failed to create menu");
    let menu = {
        let menu = gtk::Menu::new();
        menu.set_title("Junk Drawer");

        let about = gtk::MenuItem::new_with_label("About");
        menu.append(&about);
        about.show();

        menu
    };
    junk_drawer.set_popup(Some(&menu));
}
