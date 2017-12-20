use gtk;
use gtk::ContainerExt;
use gtk::CssProviderExt;
use gtk::ListBoxExt;
use gtk::ListBoxRowExt;
use gtk::ScrolledWindowExt;
use gtk::StyleContextExt;
use gtk::WidgetExt;

use res;

use std::collections::HashMap;

/// A `SectionList` is is a container that can contain section headings that can be navigated to
/// and displayed and which may have other widgets inside of them.
pub struct SectionList<'a> {
    list: gtk::ListBox,
    scroll: gtk::ScrolledWindow,
    view: gtk::Box,
    sections: HashMap<&'a str, Option<gtk::Adjustment>>,
}

impl<'a> SectionList<'a> {
    pub fn new() -> SectionList<'a> {
        let view = gtk::Box::new(gtk::Orientation::Vertical, 24);
        let window = gtk::ScrolledWindow::new(None, None);
        window.add(&view);

        return SectionList {
            list: gtk::ListBox::new(),
            scroll: window,
            view: view,
            sections: HashMap::new(),
        };
    }

    pub fn add<P: gtk::IsA<gtk::Widget>>(&mut self, s: &'a str, widget: &P) {
        let row = gtk::ListBoxRow::new();
        let label = gtk::Label::new(s);
        let ss = s.to_owned();
        row.connect_activate(move |_| {
            eprintln!("Clicked {}", ss);
        });
        if let Some(style_context) = label.get_style_context() {
            style_context.add_class("selectlabel");
            let provider = gtk::CssProvider::new();
            match provider.load_from_data(res::STYLE_LIST) {
                Ok(_) => style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION),
                Err(err) => eprintln!("error loading style provider for list: {}", err),
            }
        }
        row.add(&label);
        self.list.insert(&row, -1);
        let sep = gtk::Separator::new(gtk::Orientation::Horizontal);
        self.view.add(&sep);
        let label = gtk::Label::new(s);
        self.view.add(&label);
        if let Some(style_context) = label.get_style_context() {
            style_context.add_class("title");
            let provider = gtk::CssProvider::new();
            match provider.load_from_data(res::STYLE_LIST) {
                Ok(_) => style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION),
                Err(err) => eprintln!("error loading style provider for list: {}", err),
            }
        }
        self.view.add(widget);
        self.sections.insert(s, self.scroll.get_vadjustment());
    }

    pub fn listbox(&self) -> &gtk::ListBox {
        &self.list
    }

    pub fn view(&self) -> &gtk::ScrolledWindow {
        &self.scroll
    }
}
