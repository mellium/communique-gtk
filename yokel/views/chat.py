from yokel.gtk.view import View


class ChatView(View):

    """
    The main view which contains your open conversations and the currently
    active chat.
    """

    def __init__(self, accounts):
        super().__init__('chat_layout')
