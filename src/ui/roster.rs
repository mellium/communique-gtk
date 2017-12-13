use gtk;
use gtk::ContainerExt;
use gtk::FrameExt;
use gtk::PanedExt;
use gtk::StackSidebarExt;
use gtk::WidgetExt;

/// A widget that shows a list of the users contacts (the "roster").
pub struct Roster {
    view: gtk::Paned,
}

impl Roster {
    pub fn new() -> Roster {
        let paned = gtk::Paned::new(gtk::Orientation::Horizontal);
        let frame1 = gtk::Frame::new(None);
        let frame2 = gtk::Frame::new(None);
        frame1.set_shadow_type(gtk::ShadowType::In);
        frame2.set_shadow_type(gtk::ShadowType::In);
        paned.add1(&frame1);
        paned.add2(&frame2);

        let sidebar = gtk::StackSidebar::new();
        let stack = gtk::Stack::new();
        sidebar.set_stack(&stack);

        frame1.add(&sidebar);
        frame2.add(&stack);

        paned.show_all();

        Roster { view: paned }
    }
}

impl AsRef<gtk::Paned> for Roster {
    #[inline]
    fn as_ref(&self) -> &gtk::Paned {
        &self.view
    }
}
