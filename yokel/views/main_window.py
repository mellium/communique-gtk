from gi.repository import Gdk, GLib, Gtk
from gi.repository.Gtk import StateFlags, StyleContext

appname = "Yokel"
appauthor = "Sam Whited"


class MainWindow(Gtk.Builder):

    def __init__(self, config):
        super().__init__()

        self.using_system_theme = 'use_system_theme' in config and config[
            'use_system_theme'] is True

        settings = Gtk.Settings.get_default()
        if (self.using_system_theme and config['dark_theme'] is True):
            settings.set_property('gtk-application-prefer-dark-theme', True)

        self.add_from_file('ui/main_window.xml')

        header_bar = self.get_object('header_bar')
        header_bar.set_show_close_button(True)
        header_bar.override_color(StateFlags.NORMAL | StateFlags.ACTIVE, Gdk.RGBA(red=0, green=0, blue=1, alpha=1))

        window = self.get_object('main_window')
        window.set_titlebar(header_bar)
        window.set_position(Gtk.WindowPosition.CENTER)
        window.connect('delete-event', Gtk.main_quit)
        GLib.set_application_name(appname)
        window.set_wmclass(appname, appname)
        window.set_title(appname)
        window.set_default_size(1600, 900)

        self.mainview = self.get_object('main_view')
        self.mainbox = self.get_object('main_view_box')

        if not self.using_system_theme:
            style_provider = Gtk.CssProvider.new()
            style_provider.load_from_path('styles/headerbar.css')

            display = Gdk.Display.get_default()
            screen = display.get_default_screen()
            StyleContext.add_provider_for_screen(screen, style_provider, Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION)

        window.show_all()

        # Start the main GTK loop
        Gtk.main()
