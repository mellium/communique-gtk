from gi.repository import Gtk


class Account(Gtk.ListBoxRow):

    """
    A widget representing a single account in the accounts list.
    """

    padding = 12

    def __init__(self, account):
        super().__init__()
        self.set_name('Account')
        config = account.config

        box = Gtk.Box.new(Gtk.Orientation.HORIZONTAL, 10)

        jid = Gtk.Label.new(config['jid'])

        # close_button = Gtk.Button.new_with_label("×")
        enable_switch = Gtk.Switch.new()
        enable_switch.set_active('enabled' in config and config['enabled'])
        enable_switch.connect('state-set', account.enable_signal)

        box.pack_start(jid, False, True, self.padding)
        box.pack_end(enable_switch, False, True, self.padding)
        self.add(box)
