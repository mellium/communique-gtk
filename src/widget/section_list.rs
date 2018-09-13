use gtk;
use gtk::AdjustmentExt;
use gtk::CellLayoutExt;
use gtk::CellRendererTextExt;
use gtk::ContainerExt;
use gtk::CssProviderExt;
use gtk::GtkListStoreExtManual;
use gtk::ScrolledWindowExt;
use gtk::StyleContextExt;
use gtk::TreeViewExt;
use gtk::WidgetExt;

use glib::StaticType;

use res;

/// A `SectionList` is is a container that can contain section headings that can be navigated to
/// and displayed and which may have other widgets inside of them.
pub struct SectionList {
    list: gtk::TreeView,
    store: gtk::ListStore,
    scroll: gtk::ScrolledWindow,
    view: gtk::Box,
}

impl SectionList {
    pub fn new() -> SectionList {
        let view = gtk::Box::new(gtk::Orientation::Vertical, 24);
        let window = gtk::ScrolledWindow::new(None, None);
        window.add(&view);
        let store = gtk::ListStore::new(&[String::static_type()]);
        let tree = {
            let v = view.clone();
            let scroll = window.clone();
            let tree = gtk::TreeView::new_with_model(&store);
            tree.set_activate_on_single_click(true);
            tree.connect_row_activated(move |_, path, _| {
                let w = &v.get_children()[path.get_indices()[0] as usize];
                if let Some(cadj) = w.translate_coordinates(&v, 0, 0) {
                    let wtop = cadj.1 as f64;
                    if let Some(adj) = scroll.get_vadjustment() {
                        adj.set_value(wtop);
                    }
                }
            });
            if let Some(style_context) = tree.get_style_context() {
                let provider = gtk::CssProvider::new();
                match provider.load_from_data(res::STYLE_LIST) {
                    Ok(_) => style_context
                        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION),
                    Err(err) => eprintln!("error loading style provider for tree: {}", err),
                }
            }
            tree.set_headers_visible(false);
            let column = gtk::TreeViewColumn::new();
            let cell = gtk::CellRendererText::new();
            cell.set_property_weight(600);
            column.pack_start(&cell, true);
            column.add_attribute(&cell, "text", 0);
            tree.append_column(&column);

            tree
        };

        return SectionList {
            list: tree,
            store: store,
            scroll: window,
            view: view,
        };
    }

    pub fn add<P: gtk::IsA<gtk::Widget>>(&mut self, s: &str, widget: &P) {
        self.store.insert_with_values(None, &[0], &[&s.to_owned()]);

        let w = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let sep = gtk::Separator::new(gtk::Orientation::Horizontal);
        w.add(&sep);
        let label = gtk::Label::new(s);
        w.add(&label);
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
        w.add(widget);

        self.view.add(&w);
    }

    pub fn listbox(&self) -> &gtk::TreeView {
        &self.list
    }

    pub fn view(&self) -> &gtk::ScrolledWindow {
        &self.scroll
    }
}
