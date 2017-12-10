use glib::Cast;
use res;
use ui;

use gtk;
use gtk::BoxExt;
use gtk::ContainerExt;
use gtk::GtkWindowExt;

/// The main application window ties together the header bar, various views, and menus.
pub struct AppWindow {
    view: gtk::ApplicationWindow,
    container: gtk::Box,
}

impl AppWindow {
    /// Creates the main application window and populates it with a header bar and a view area
    /// where the various application panes can be rendered.
    pub fn new(app: &gtk::Application) -> AppWindow {
        let window = gtk::ApplicationWindow::new(app);

        window.set_title(res::APP_NAME);
        window.set_default_size(350, 70);
        window.set_position(gtk::WindowPosition::Center);

        let bar = ui::main::header_bar(&window.clone().upcast::<gtk::Window>());
        window.set_titlebar(&bar);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        window.add(&container);

        return AppWindow {
            view: window,
            container: container,
        };
    }

    /// Sets the main view of the application window.
    pub fn set_view<T: gtk::IsA<gtk::Widget>>(&self, widget: &T) {
        for ref w in self.container.get_children() {
            self.container.remove(w);
        }

        self.container.add(widget);
        self.container.set_child_packing(
            widget,
            true,
            true,
            0,
            gtk::PackType::Start,
        );
    }
}

impl AsRef<gtk::ApplicationWindow> for AppWindow {
    #[inline]
    fn as_ref(&self) -> &gtk::ApplicationWindow {
        return &self.view;
    }
}
