use gtk;
use gtk::ContainerExt;
use gtk::FlowBoxExt;
use gtk::FrameExt;
use gtk::PanedExt;
use gtk::StackExt;
use gtk::StackSidebarExt;
use gtk::WidgetExt;

use widget;

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

        let scroll = gtk::ScrolledWindow::new(None, None);
        let flow = gtk::FlowBox::new();
        flow.set_property_homogeneous(true);
        scroll.add(&flow);
        stack.add_titled(&scroll, "All", "All");

        vec![
            "Beautiful",
            "Catchup",
            "Dandelion",
            "Deadwood",
            "Fuego Borrego",
            "Green Giant",
            "Mailman",
            "Mississippi Mule",
            "Papa Shrimp",
            "Pockets",
            "Poison Ivey",
            "Shrimpette",
            "Spoon Foot",
            "Stoked",
            "Sunshine",
            "Thespian",
            "Twinkletoes",
            "Utah Red",
            "Wired",
            "Zodiac",
        ].iter().for_each(|name| {
            let c = widget::avatar(name, None);
            flow.add(&c);
        });

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
