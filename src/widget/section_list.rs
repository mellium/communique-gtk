use gtk;
use gtk::ContainerExt;
use gtk::ListBoxExt;
use gtk::ScrolledWindowExt;

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
        let label = gtk::Label::new(s);
        self.list.insert(&label, -1);
        let label = gtk::Label::new(s);
        self.view.add(&label);
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
