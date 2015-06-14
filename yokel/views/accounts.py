from yokel.gtk.account import Account
from yokel.gtk.view import View


class AccountsView(View):

    """
    A view representing a list of accounts and their preferences.
    """

    def __init__(self, accounts):
        super().__init__('accounts_layout')

        accounts_list = self.builder.get_object('accounts_list')

        for account in accounts:
            accounts_list.prepend(Account(account))
