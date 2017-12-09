use gtk;
use gtk::MenuButtonExt;
use gtk::MenuExt;
use gtk::MenuShellExt;
use gtk::WidgetExt;

/// The junk drawer is the main application menu.
///
/// It is visible whenever we can see the titlebar and contains options that control the entire
/// application.
/// The less you put in the junk drawer the better. If you're unsure, then it doesn't belong here.
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
