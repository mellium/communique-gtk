import signal

from yokel.config import Config
from yokel.views.main_window import MainWindow

c = Config({
    'dark_theme': True,
    'use_system_theme': False
})

signal.signal(signal.SIGINT, signal.SIG_DFL)

MainWindow(c)
c.flush()
