use gdk_pixbuf;

use gtk;
use gtk::ContainerExt;
use gtk::FrameExt;
use gtk::ListBoxExt;
use gtk::PanedExt;
use gtk::StackSidebarExt;
use gtk::WidgetExt;

use std::collections::hash_map::DefaultHasher;
use std::f64;
use std::hash::{Hash, Hasher};

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
        let list = gtk::ListBox::new();
        scroll.add(&list);
        stack.add(&scroll);


        let names = vec![
            "Beautiful",
            "Catchup",
            "Dandelion",
            "Fuego Borrego",
            "Green Giant",
            "Mailman",
            "Papa Shrimp",
            "Pockets",
            "Spoon Foot",
            "Sunshine",
            "Thespian",
            "Twinkle Toes",
            "Zodiac",
        ];
        names.iter().inspect(|name| {
            let lbr = gtk::ListBoxRow::new();
            let c = contact(name, None);
            lbr.add(&c);
            list.insert(&lbr, -1);
        }).count();

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

const KR: f64 = 0.299;
const KG: f64 = 0.587;
const KB: f64 = 0.114;
const Y: f64 = 0.731;

fn contact<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(name: &str, _avatar: P) -> gtk::Box {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 12);

    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let h = hasher.finish();
    let angle = ((h & (0xffff << 48)) >> 48) as f64 / 65536.0 * 2.0 * f64::consts::PI;
    let (cb, cr) = angle.sin_cos();
    let factor = cr.abs().max(cb.abs());
    let (cb, cr) = (cb * factor, cr * factor);
    let r = 2.0 * (1.0 - KR) * cr + Y;
    let b = 2.0 * (1.0 - KB) * cb + Y;
    let g = (Y - KR * r - KB * b) / KG;

    let drawing = gtk::DrawingArea::new();
    drawing.set_size_request(128, 128);
    drawing.connect_draw(move |_, ctx| {
        ctx.set_source_rgba(r, g, b, 1.0);
        ctx.paint();
        gtk::Inhibit(true)
    });

    let name_label = gtk::Label::new(name);

    vbox.add(&drawing);
    vbox.add(&name_label);
    vbox
}
