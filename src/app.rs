use config;
use gdk_pixbuf;
use glib;
use res;
use ui;

use gdk;
use gdk::DisplayExt;

use gio;
use gio::ActionMapExt;
use gio::ApplicationExt;
use gio::ApplicationExtManual;
use gio::SimpleActionExt;

use gtk;
use gtk::BoxExt;
use gtk::ContainerExt;
use gtk::CssProviderExt;
use gtk::DialogExt;
use gtk::GtkApplicationExt;
use gtk::GtkWindowExt;
use gtk::SettingsExt;
use gtk::StackSwitcherExt;
use gtk::WidgetExt;

use std::convert::From;

/// Possible error values that can occur when creating applications.
#[derive(Debug)]
pub enum Error {
    Bool(glib::error::BoolError),
    Glib(glib::Error),
}

impl From<glib::Error> for Error {
    fn from(err: glib::Error) -> Self {
        Error::Glib(err)
    }
}

impl From<glib::error::BoolError> for Error {
    fn from(err: glib::error::BoolError) -> Self {
        Error::Bool(err)
    }
}

/// The app handles creating the UI and reacting to GIO signals.
pub struct App {
    app: gtk::Application,
}

impl App {
    /// Creates the main application and reacts to the GIO activation signal to populate a window
    /// with a header bar and a view area where the various application panes can be rendered.
    pub fn new() -> Result<App, Error> {
        let app = gtk::Application::new(Some(res::APP_ID), gio::ApplicationFlags::FLAGS_NONE)?;
        let config = config::load_config();
        let me = App { app: app };

        let display = gdk::Display::get_default().unwrap();
        let screen = display.get_default_screen();
        let style_provider = gtk::CssProvider::new();
        style_provider.load_from_data(res::STYLE_MAIN)?;

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
                style_provider.load_from_data(res::STYLE_CONVERSATIONS)?;
                gtk::StyleContext::add_provider_for_screen(
                    &screen,
                    &style_provider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }
            _ => {}
        }

        me.app.connect_startup(|_| {});
        me.app.connect_activate(clone!(config => move |app| {
            let window = gtk::ApplicationWindow::new(app);
            let logoloader = gdk_pixbuf::PixbufLoader::new();
            logoloader.loader_write(res::SVG_LOGO).unwrap();
            logoloader.close().unwrap();
            let logobuf = logoloader.get_pixbuf().unwrap();

            // Quit action
            let close = gio::SimpleAction::new("close", None);
            close.connect_activate(clone!( window => move |_, _| {
                window.destroy();
            }));
            app.add_action(&close);

            // About action
            let about = gio::SimpleAction::new("about", None);
            about.connect_activate(clone!( window, logobuf => move |_, _| {
                let p = ui::about_dialog(&window, &logobuf);
                p.run();
                p.destroy();
            }));
            app.add_action(&about);

            window.set_title(res::APP_NAME);
            window.set_default_size(1280, 720);
            window.set_position(gtk::WindowPosition::Center);

            // Show the application menu in the application or the titlebar depending on the desktop
            // environments preference.
            let menu = ui::app_menu();

            let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
            window.add(&container);

            let switcher: Option<gtk::StackSwitcher>;
            if config.accounts.is_empty() {
                let login_pane = ui::Login::new(&logobuf);
                switcher = None;
                App::set_stack(&container, None, login_pane.as_ref());
                login_pane.grab_default();
            } else {
                let chat_pane = ui::Chat::new();
                switcher = Some(gtk::StackSwitcher::new());
                App::set_stack(&container, switcher.as_ref(), chat_pane.as_ref());
            }

            let hbar = if app.prefers_app_menu() {
                app.set_app_menu(&menu);
                ui::header_bar(None, switcher.as_ref())
            } else {
                ui::header_bar(Some(&menu), switcher.as_ref())
            };
            window.set_titlebar(&hbar);

            app.add_window(&window);
            window.show_all();
        }));


        me.app.register(None)?;
        Ok(me)
    }

    /// Sets the main view of the application window.
    fn set_stack<'a, T: Into<Option<&'a gtk::StackSwitcher>>>(
        container: &gtk::Box,
        switcher: T,
        stack: &gtk::Stack,
    ) {
        for w in container.get_children() {
            container.remove(&w);
        }

        if let Some(switcher) = switcher.into() {
            switcher.set_stack(stack);
        }
        container.add(stack);
        container.set_child_packing(stack, true, true, 0, gtk::PackType::Start);
    }

    /// Run the application.
    pub fn run(&self, argv: &[String]) -> i32 {
        self.app.run(argv)
    }
}

impl AsRef<gtk::Application> for App {
    #[inline]
    fn as_ref(&self) -> &gtk::Application {
        &self.app
    }
}
