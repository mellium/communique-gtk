import logging

version = '0.0.0'
"""
The version number of Yokel.
"""

appname = 'Yokel'
"""
The name of this app.
"""

appauthor = "Sam Whited"
"""
Author info for this app.
"""

log = logging.getLogger(__name__)
log.propagate = False
log.setLevel(logging.DEBUG)

try:
    from systemd.journal import JournalHandler
    log.addHandler(JournalHandler(SYSLOG_IDENTIFIER=__name__))
except:
    # journald is not available on this system.
    pass
