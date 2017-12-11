use gio;
use res;

use gtk;
use gtk::ButtonBoxExt;
use gtk::ButtonExt;
use gtk::ContainerExt;
use gtk::HeaderBarExt;
use gtk::MenuButtonExt;
use gtk::ToggleButtonExt;
use gtk::WidgetExt;

/// Constructs and populates the main header bar.
pub fn header_bar(appmenu: Option<&gio::Menu>) -> gtk::HeaderBar {
    let bar = gtk::HeaderBar::new();

    bar.set_title(res::APP_NAME);

    let buttons = title_buttons();
    bar.set_custom_title(&buttons);

    if let Some(menu) = appmenu {
        let context_button = gtk::MenuButton::new();
        context_button.set_label("❧");
        context_button.set_menu_model(menu);
        bar.add(&context_button);
    }

    let new_button =
        gtk::Button::new_from_icon_name("list-add-symbolic", gtk::IconSize::Button.into());
    new_button.set_sensitive(false);
    bar.add(&new_button);

    let search = gtk::SearchEntry::new();
    search.set_sensitive(false);
    bar.pack_end(&search);

    return bar;
}

fn title_buttons() -> gtk::ButtonBox {
    let bbox = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    bbox.set_layout(gtk::ButtonBoxStyle::Center);
    bbox.set_sensitive(false);

    let roster = gtk::RadioButton::new_with_label(translate!("Roster"));
    let conversations =
        gtk::RadioButton::new_with_label_from_widget(&roster, translate!("Conversations"));

    roster.set_property_draw_indicator(false);
    conversations.set_property_draw_indicator(false);

    bbox.add(&roster);
    bbox.add(&conversations);

    return bbox;
}
