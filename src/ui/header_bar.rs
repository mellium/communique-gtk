use gio;
use res;

use gtk;
use gtk::ActionableExt;
use gtk::ButtonExt;
use gtk::ContainerExt;
use gtk::HeaderBarExt;
use gtk::MenuButtonExt;
use gtk::WidgetExt;

/// Constructs and populates the main header bar.
pub fn header_bar(appmenu: Option<&gio::Menu>) -> gtk::HeaderBar {
    let hbar = gtk::HeaderBar::new();

    hbar.set_title(res::APP_NAME);

    if let Some(menu) = appmenu {
        let context_button = gtk::MenuButton::new();
        context_button.set_label("❧");
        context_button.set_menu_model(menu);
        hbar.add(&context_button);
    }

    let new_button =
        gtk::Button::new_from_icon_name("list-add-symbolic", gtk::IconSize::Button.into());
    new_button.set_sensitive(false);
    hbar.add(&new_button);

    let search =
        gtk::Button::new_from_icon_name("system-search-symbolic", gtk::IconSize::Button.into());
    search.set_action_name("win.search");
    hbar.pack_end(&search);

    hbar
}
