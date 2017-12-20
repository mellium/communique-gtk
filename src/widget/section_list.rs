use gtk;
use gtk::CellLayoutExt;
use gtk::CellRendererTextExt;
use gtk::ContainerExt;
use gtk::CssProviderExt;
use gtk::ListStoreExtManual;
use gtk::ScrolledWindowExt;
use gtk::StyleContextExt;
use gtk::TreeViewExt;
use gtk::WidgetExt;

use glib::StaticType;

use res;

use std::collections::HashMap;

/// A `SectionList` is is a container that can contain section headings that can be navigated to
/// and displayed and which may have other widgets inside of them.
pub struct SectionList<'a> {
    list: gtk::TreeView,
    store: gtk::ListStore,
    scroll: gtk::ScrolledWindow,
    view: gtk::Box,
    sections: HashMap<&'a str, Option<gtk::Adjustment>>,
}

impl<'a> SectionList<'a> {
    pub fn new() -> SectionList<'a> {
        let view = gtk::Box::new(gtk::Orientation::Vertical, 24);
        let window = gtk::ScrolledWindow::new(None, None);
        window.add(&view);
        let store = gtk::ListStore::new(&[String::static_type()]);
        let tree = gtk::TreeView::new_with_model(&store);
        tree.set_headers_visible(false);
        let column = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();
        cell.set_property_weight(600);
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        tree.append_column(&column);

        return SectionList {
            list: tree,
            store: store,
            scroll: window,
            view: view,
            sections: HashMap::new(),
        };
    }

    pub fn add<P: gtk::IsA<gtk::Widget>>(&mut self, s: &'a str, widget: &P) {
        self.store.insert_with_values(None, &[0], &[&s.to_owned()]);

        let sep = gtk::Separator::new(gtk::Orientation::Horizontal);
        self.view.add(&sep);
        let label = gtk::Label::new(s);
        self.view.add(&label);
        if let Some(style_context) = label.get_style_context() {
            style_context.add_class("title");
            let provider = gtk::CssProvider::new();
            match provider.load_from_data(res::STYLE_LIST) {
                Ok(_) => {
                    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION)
                }
                Err(err) => eprintln!("error loading style provider for list: {}", err),
            }
        }
        self.view.add(widget);
        self.sections.insert(s, self.scroll.get_vadjustment());
    }

    pub fn listbox(&self) -> &gtk::TreeView {
        &self.list
    }

    pub fn view(&self) -> &gtk::ScrolledWindow {
        &self.scroll
    }
}
