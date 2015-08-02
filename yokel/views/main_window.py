from gi.repository import Gdk, GLib, Gtk
from gi.repository.Gtk import StateFlags, StyleContext

from yokel import appname
from yokel.views.accounts import AccountsView
from yokel.views.chat import ChatView


class MainWindow(Gtk.Builder):

    """
    The main window of the app. Controls creation of the presenters and handing
    them application state. This should probably be separated somehow.

    Args:
        config (dict): The global config object.
    """

    def __init__(self, config, app_state):
        super().__init__()

        self.config = config
        self.app_state = app_state

        self.primary_view = None

        self.using_system_theme = 'use_system_theme' in config and config[
            'use_system_theme'] is True

        settings = Gtk.Settings.get_default()
        if self.using_system_theme:
            settings.set_property('gtk-application-prefer-dark-theme',
                                  config['dark_theme'])

        self.add_from_file('ui/main_window.xml')

        header_bar = self.get_object('header_bar')
        header_bar.set_show_close_button(True)
        header_bar.override_color(StateFlags.NORMAL | StateFlags.ACTIVE,
                                  Gdk.RGBA(red=0, green=0, blue=1, alpha=1))

        window = self.get_object('main_window')
        window.set_titlebar(header_bar)
        window.set_position(Gtk.WindowPosition.CENTER)
        window.connect('delete-event', Gtk.main_quit)
        GLib.set_application_name(appname)
        window.set_wmclass(appname, appname)
        window.set_title(appname)
        window.set_default_size(1600, 900)

        self.mainbox = self.get_object('main_view_box')
        self.window = window

        if not self.using_system_theme and 'theme' in config:
            style_provider = Gtk.CssProvider.new()
            style_provider.load_from_path(
                'styles/{}.css'.format(config['theme'])
            )

            display = Gdk.Display.get_default()
            screen = display.get_default_screen()
            StyleContext.add_provider_for_screen(
                screen,
                style_provider,
                Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
            )

        self.accounts_view = AccountsView(self.app_state['accounts'])
        self.chat_view = ChatView(self.app_state['accounts'])
        if len(app_state['accounts']) == 0 or len(
                app_state['accounts'].enabled) == 0:
            self.view_accounts()
        else:
            self.view_accounts()
            # self.view_chat()

    def view_accounts(self):
        """
        Loads the accounts view in the window.
        """
        if type(self.primary_view) == AccountsView:
            return

        self.set_primary_view(self.accounts_view)

    def view_chat(self):
        """
        Loads the chat view in the window.
        """
        if type(self.primary_view) == ChatView:
            return

        self.set_primary_view(self.chat_view)

    def set_primary_view(self, view):
        """
        Sets the primary view of the window.
        """
        if self.primary_view:
            self.mainbox.remove(self.primary_view)
        self.primary_view = view
        self.mainbox.add(self.primary_view)
        self.window.show_all()
