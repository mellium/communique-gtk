use gtk;
use gtk::ContainerExt;
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

        let mut sections = widget::SectionList::new();
        {
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
            sections.add(translate!("Roster"), &flow);
        }
        {
            let flow = gtk::FlowBox::new();
            vec![
                "AT Class of 2014",
                "Cars & Coffee",
                "Conversations",
                "Mellium Devs",
                "XMPP Council Room",
                "XSF Discussion Room",
            ].iter()
                .for_each(|name| {
                    let c = widget::avatar(name, None);
                    flow.add(&c);
                });
            sections.add(translate!("Conferences"), &flow);
        }
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
