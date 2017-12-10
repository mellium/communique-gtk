use res;

use gtk;
use gtk::AboutDialogExt;
use gtk::ButtonBoxExt;
use gtk::ButtonExt;
use gtk::ContainerExt;
use gtk::DialogExt;
use gtk::GtkWindowExt;
use gtk::HeaderBarExt;
use gtk::MenuButtonExt;
use gtk::MenuItemExt;
use gtk::MenuShellExt;
use gtk::ToggleButtonExt;
use gtk::WidgetExt;

/// Constructs and populates the main header bar.
pub fn header_bar(window: &gtk::Window) -> gtk::HeaderBar {
    let bar = gtk::HeaderBar::new();

    bar.set_has_subtitle(false);
    bar.set_title(res::APP_NAME);

    let buttons = title_buttons();
    bar.set_custom_title(&buttons);

    let context_button = gtk::MenuButton::new();
    context_button.set_label("❧");
    let menu = {
        let menu = gtk::Menu::new();

        let about = gtk::MenuItem::new_with_label(translate!("About"));
        let quit = gtk::MenuItem::new_with_label(translate!("Quit"));

        about.connect_activate(clone!(window => move |_| {
            let p = gtk::AboutDialog::new();
            p.set_website_label(Some("gtk-rs"));
            p.set_website(Some("http://gtk-rs.org"));
            p.set_authors(&["Gtk-rs developers"]);
            p.set_title(translate!("About"));
            p.set_transient_for(Some(&window));
            p.run();
            p.destroy();
        }));
        quit.connect_activate(clone!(window => move |_| {
            window.destroy();
        }));

        menu.append(&about);
        menu.append(&quit);

        about.show();
        quit.show();

        menu
    };
    context_button.set_popup(&menu);
    bar.add(&context_button);

    let new_button =
        gtk::Button::new_from_icon_name("list-add-symbolic", gtk::IconSize::Button.into());
    bar.add(&new_button);

    let search = gtk::SearchEntry::new();
    bar.pack_end(&search);

    return bar;
}

fn title_buttons() -> gtk::ButtonBox {
    let bbox = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    bbox.set_layout(gtk::ButtonBoxStyle::Center);

    let roster = gtk::RadioButton::new_with_label(translate!("Roster"));
    let conversations =
        gtk::RadioButton::new_with_label_from_widget(&roster, translate!("Conversations"));

    roster.set_property_draw_indicator(false);
    conversations.set_property_draw_indicator(false);

    bbox.add(&roster);
    bbox.add(&conversations);

    return bbox;
}
