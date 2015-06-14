import signal

from gi.repository import Gtk, GObject

from yokel.config import Config
from yokel.models.accounts import AccountManager
from yokel.views.main_window import MainWindow

signal.signal(signal.SIGINT, signal.SIG_DFL)

config = Config({
    'dark_theme': False,
    'use_system_theme': True,
    'theme': 'conversations',
    'account': [],
})

app_state = {
    'accounts': AccountManager(config['account']),
}

# Unnecessary for PyGObject 3.10.2+
GObject.threads_init()

MainWindow(config, app_state)

Gtk.main()
config.flush()
