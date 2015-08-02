import asyncio

from gi.repository import Gtk
from yokel import log
from slixmpp import ClientXMPP


class AccountManager(Gtk.ListStore):

    """
    Maintains state for all XMPP accounts.

    Args:
        config (dict): The account key of the global config dict.
    """

    def __init__(self, config):
        super().__init__()
        self.accounts = [Account(account) for account in config]

    def __iter__(self):
        return (account for account in self.accounts)

    def __len__(self):
        return self.accounts.__len__()

    @property
    def enabled(self):
        """
        A list of enabled accounts.
        """
        return [account for account in self.accounts if
                account.enabled is True]

    @property
    def config(self):
        """
        Return the updated account config.
        """
        return [account.config for account in self.accounts]


class Account(ClientXMPP):

    """
    Represents a single XMPP account.

    Args:
        config (dict): The account subsection of the main config dict.
        password (str): The password for the account.
    Raises:
        KeyError: When no `jid` key is supplied in the config.
    """

    def __init__(self, config, password=None):
        self.config = {
            'enabled': False
        }
        self.config.update(config)
        self.loop = asyncio.get_event_loop()

        super().__init__(
            self.config['jid'],
            password=password or self.config['password']
        )
        self.register_plugin('xep_0092')

        self.add_event_handler("session_start", self.session_start)

    @property
    def enabled(self):
        """
        True if the account is enabled.
        """
        return self.config['enabled']

    def connect(self):
        """
        Connect to the XMPP server using the provided config.
        """
        log.info('Connecting to {}'.format(
            self.config['jid']
        ))
        super().connect(force_starttls=True)

    def session_start(self):
        """
        Begins an XMPP session by sending presence info and fetching the
        roster.
        """
        log.debug('Starting a session for {}'.format(
            self.config['jid']
        ))
        self.send_presence()
        self.get_roster()

    def enable_signal(self, button, value):
        """
        Enable or disable the account.
        """
        log.debug('Received account {} signal for {}'.format(
            value and 'enable' or 'disable',
            self.config['jid']
        ))
        self.config['enabled'] = value

        if value:
            self.connect()
