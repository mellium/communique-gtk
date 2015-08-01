import asyncio
import signal
import sys
import threading

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

try:
    loop = asyncio.get_event_loop()

    # Unnecessary for PyGObject 3.10.2+
    GObject.threads_init()

    MainWindow(config, app_state)

    def start_ui(loop=loop):
        Gtk.main()
        loop.call_soon_threadsafe(loop.stop)
        sys.exit(0)

    thread = threading.Thread(target=start_ui)
    thread.start()

    loop.run_forever()
finally:
    loop.close()
    config['account'] = app_state['accounts'].config
    config.flush()
