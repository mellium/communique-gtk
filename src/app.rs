use config;
use gdk_pixbuf;
use glib::Cast;
use res;
use ui;
use widget;

use gdk;
use gdk::DisplayExt;

use gio;
use gio::ApplicationExt;
use gio::ApplicationExtManual;

use gtk;
use gtk::BoxExt;
use gtk::ContainerExt;
use gtk::CssProviderExt;
use gtk::GtkApplicationExt;
use gtk::GtkWindowExt;
use gtk::SettingsExt;
use gtk::WidgetExt;

/// The app handles creating the UI and reacting to GIO signals.
pub struct App {
    app: gtk::Application,
}

impl App {
    /// Creates the main application and reacts to the GIO activation signal to populate a window
    /// with a header bar and a view area where the various application panes can be rendered.
    pub fn new() -> App {
        let app = gtk::Application::new(Some(res::APP_ID), gio::ApplicationFlags::FLAGS_NONE)
            .unwrap();
        let config = config::load_config();

        app.connect_startup(clone!(config => move |_| {
            let display = gdk::Display::get_default().unwrap();
            let screen = display.get_default_screen();
            let style_provider = gtk::CssProvider::new();
            style_provider.load_from_data(res::STYLE_MAIN).unwrap();

            match config.theme.as_ref().map(|s| s.as_ref()) {
                Some("dark") => {
                    if let Some(settings) = gtk::Settings::get_default() {
                        settings.set_property_gtk_application_prefer_dark_theme(true);
                    }
                }
                Some("light") => {
                    if let Some(settings) = gtk::Settings::get_default() {
                        settings.set_property_gtk_application_prefer_dark_theme(false);
                    }
                }
                Some("conversations") => {
                    style_provider
                        .load_from_data(res::STYLE_CONVERSATIONS)
                        .unwrap();
                    gtk::StyleContext::add_provider_for_screen(
                        &screen,
                        &style_provider,
                        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                    );
                }
                _ => {}
            }
        }));
        app.connect_activate(clone!(config => move |app| {
            let window = gtk::ApplicationWindow::new(&app);

            window.set_title(res::APP_NAME);
            window.set_default_size(350, 70);
            window.set_position(gtk::WindowPosition::Center);

            let logoloader = gdk_pixbuf::PixbufLoader::new();
            logoloader.loader_write(res::SVG_LOGO).unwrap();
            logoloader.close().unwrap();
            let logobuf = logoloader.get_pixbuf().unwrap();

            let bar = ui::main::header_bar(&window.clone().upcast::<gtk::Window>(), &logobuf);
            window.set_titlebar(&bar);

            let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
            window.add(&container);
            if config.accounts.len() == 0 {
                let login_pane = widget::Login::new(&logobuf);
                App::set_view(&container, login_pane.as_ref());
                login_pane.grab_default();
            }

            app.add_window(&window);
            window.show_all();
        }));


        return App { app: app };
    }

    /// Sets the main view of the application window.
    fn set_view<T: gtk::IsA<gtk::Widget>>(container: &gtk::Box, widget: &T) {
        for ref w in container.get_children() {
            container.remove(w);
        }

        container.add(widget);
        container.set_child_packing(widget, true, true, 0, gtk::PackType::Start);
    }

    /// Registers the application with GIO.
    pub fn register<'a, P: Into<Option<&'a gio::Cancellable>>>(
        &self,
        cancelable: P,
    ) -> Result<(), gio::Error> {
        self.app.register(cancelable)
    }

    /// Run the application.
    pub fn run(&self, argv: &[String]) -> i32 {
        self.app.run(argv)
    }
}

impl AsRef<gtk::Application> for App {
    #[inline]
    fn as_ref(&self) -> &gtk::Application {
        return &self.app;
    }
}
