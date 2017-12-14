use gdk::ContextExt;

use gdk_pixbuf;

use gtk;
use gtk::ContainerExt;
use gtk::FrameExt;
use gtk::GridExt;
use gtk::PanedExt;
use gtk::StackExt;
use gtk::StackSidebarExt;
use gtk::StyleContextExt;
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
            let c = contact(name, None);
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

const KR: f64 = 0.299;
const KG: f64 = 0.587;
const KB: f64 = 0.114;
const Y: f64 = 0.731;
const BLEND_FACTOR: f64 = 0.2;

fn contact<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(name: &str, avatar: P) -> gtk::Box {
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
    if let Some(pic) = avatar.into() {
        drawing.connect_draw(clone!( pic => move |_, ctx| {
            ctx.set_source_pixbuf(&pic, 0.0, 0.0);
            ctx.paint();
            gtk::Inhibit(true)
        }));
    } else {
        drawing.connect_draw(move |widget, ctx| {
            // If we can get a style_context, blend the color with the background.
            let (rb, bb, gb) = match widget.get_style_context() {
                Some(style_context) => {
                    let bg = style_context.get_background_color(style_context.get_state());
                    let r = BLEND_FACTOR * (1.0 - bg.red) + (1.0 - BLEND_FACTOR) * r;
                    let b = BLEND_FACTOR * (1.0 - bg.blue) + (1.0 - BLEND_FACTOR) * b;
                    let g = BLEND_FACTOR * (1.0 - bg.green) + (1.0 - BLEND_FACTOR) * g;
                    (r, b, g)
                }
                None => (r, b, g),
            };
            ctx.set_source_rgb(rb, gb, bb);
            ctx.paint();
            gtk::Inhibit(true)
        });
    }

    let name_label = gtk::Label::new(name);

    vbox.add(&drawing);
    vbox.add(&name_label);
    vbox
}
