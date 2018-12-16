use gtk;
use gtk::StackExt;
use gtk::StackSwitcherExt;
use gtk::WidgetExt;

use crate::ui::roster::Roster;

/// The main chat and roster view.
pub struct Chat {
    stack: gtk::Stack,
    switcher: gtk::StackSwitcher,
}

impl Chat {
    /// Creates a new chat pane that can be shown in the application.
    pub fn new() -> Chat {
        let stack = gtk::Stack::new();
        let switcher = gtk::StackSwitcher::new();
        switcher.set_stack(&stack);

        let roster = Roster::new();
        let chats = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let roster_name = translate!("Roster");
        let chats_name = translate!("Conversations");

        stack.add_titled(roster.as_ref(), roster_name, roster_name);
        stack.add_titled(&chats, chats_name, chats_name);

        stack.show_all();
        switcher.show_all();

        Chat {
            stack: stack,
            switcher: switcher,
        }
    }

    /// Gets the underlying stack switcher.
    pub fn switcher(&self) -> &gtk::StackSwitcher {
        &self.switcher
    }
}

impl AsRef<gtk::Stack> for Chat {
    #[inline]
    fn as_ref(&self) -> &gtk::Stack {
        &self.stack
    }
}
