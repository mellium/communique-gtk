use gio;
use gio::MenuExt;

/// The main application menu which may be shown in either the shell or in the header bar.
pub fn app_menu() -> gio::Menu {
    let menu = gio::Menu::new();

    menu.append(translate!("About"), "app.about");
    menu.append(translate!("Close"), "app.close");

    menu
}
