use gtk;
use gtk::ContainerExt;
use gtk::FlowBoxExt;
use gtk::FrameExt;
use gtk::PanedExt;
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

        let flow = gtk::FlowBox::new();
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
        ].iter()
            .for_each(|name| {
                let c = widget::avatar(name, None);
                flow.add(&c);
            });
        flow.set_property_homogeneous(true);
        let flow2 = gtk::FlowBox::new();
        vec![
            "Mailman",
            "Beautiful",
            "Fuego Borrego",
            "Green Giant",
            "Deadwood",
            "Dandelion",
            "Catchup",
        ].iter()
            .for_each(|name| {
                let c = widget::avatar(name, None);
                flow2.add(&c);
            });
        flow2.set_property_homogeneous(true);
        let mut sections = widget::SectionList::new();
        sections.add(translate!("Roster"), &flow);
        sections.add(translate!("Conferences"), &flow2);
        let scroll = sections.view();
        let list = sections.listbox();

        frame1.add(list);
        frame2.add(scroll);

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
