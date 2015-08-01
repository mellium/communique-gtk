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

    @property
    def enabled_accounts(self):
        """
        A list of enabled accounts.
        """
        return [account for account in self.accounts if 'enabled' in account
                and account['enabled'] is True]

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
        password (basestring): The password for the account.
    Raises:
        KeyError: When no `jid` key is supplied in the config.
    """

    def __init__(self, config, password=None):
        self.config = {
            'enabled': False
        }
        self.config.update(config)

        super().__init__(self.config['jid'], password=password)

        if 'enabled' in config and config['enabled']:
            self.connect()

    def connect(self):
        """
        Connect to the XMPP server using the provided config.
        """
        log.info('Connecting to {}'.format(
            self.config['jid']
        ))
        super().connect(
            force_starttls=True,
        )

    def enable_signal(self, button, value):
        """
        Enable or disable the account.
        """
        log.info('Received account {} signal for {}'.format(
            value and 'enable' or 'disable',
            self.config['jid']
        ))
        self.config['enabled'] = value

        if value:
            self.connect()
