import signal

from gi.repository import Gtk

from yokel.config import Config
from yokel.views.main_window import MainWindow

c = Config({
    'dark_theme': False,
    'use_system_theme': True,
    'theme': 'conversations'
})

signal.signal(signal.SIGINT, signal.SIG_DFL)

MainWindow(c)
Gtk.main()
c.flush()
