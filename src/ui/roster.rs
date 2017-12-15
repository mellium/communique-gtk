use gtk;
use gtk::ContainerExt;
use gtk::FrameExt;
use gtk::GridExt;
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
        let grid = gtk::Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);
        grid.set_row_spacing(12);
        grid.set_column_spacing(12);
        grid.insert_column(0);
        grid.insert_column(1);
        grid.insert_column(2);
        scroll.add(&grid);
        stack.add_titled(&scroll, "All", "All");

        let names = vec![
            "Beautiful",
            "Catchup",
            "Dandelion",
            "Fuego Borrego",
            "Green Giant",
            "Mailman",
            "Papa Shrimp",
            "Pockets",
            "Poison Ivey",
            "Shrimpette",
            "Spoon Foot",
            "Sunshine",
            "Thespian",
            "Twinkle Toes",
            "Utah Red",
            "Wired",
            "Zodiac",
        ];
        let mut x = 0;
        let mut y = 0;
        names.iter().for_each(|name| {
            let c = widget::avatar(name, None);
            grid.attach(&c, x, y, 1, 1);
            x += 1;
            if x % 3 == 0 {
                y += 1;
                x = 0;
            }
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
