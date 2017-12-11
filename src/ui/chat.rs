use gtk;
use gtk::StackExt;

/// The main chat and roster view.
pub struct Chat {
    stack: gtk::Stack,
}

impl Chat {
    /// Creates a new chat pane that can be shown in the application.
    pub fn new() -> Chat {
        let stack = gtk::Stack::new();

        let roster = gtk::Frame::new(None);
        let roster_name = translate!("Roster");

        let chats = gtk::Frame::new(None);
        let chats_name = translate!("Conversations");

        stack.add_titled(&roster, roster_name, roster_name);
        stack.add_titled(&chats, chats_name, chats_name);

        Chat { stack: stack }
    }
}

impl AsRef<gtk::Stack> for Chat {
    #[inline]
    fn as_ref(&self) -> &gtk::Stack {
        &self.stack
    }
}
