import asyncio
import signal
import sys

from gi.repository import Gdk, Gtk, GObject

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

try:
    # Unnecessary for PyGObject 3.10.2+
    GObject.threads_init()

    MainWindow(config, app_state)

    Gdk.threads_init()
    ui_loop = asyncio.new_event_loop()
    asyncio.set_event_loop(ui_loop)
    ui_loop.run_in_executor(None, None)
    Gtk.main()
    Gdk.threads_leave()
finally:
    # ui_loop.call_soon(ui_loop.stop)
    # ui_loop.call_soon(ui_loop.close)
    config['account'] = app_state['accounts'].config
    config.flush()
