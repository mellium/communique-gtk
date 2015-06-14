from gi.repository import Gtk


class View(Gtk.ScrolledWindow):

    """
    A generic view representation which is loaded from a glade file.
    """

    def __init__(self, layout_name):
        super().__init__()

        self.builder = Gtk.Builder.new_from_file(
            'ui/{}.xml'.format(layout_name)
        )
        for gtkobj in self.builder.get_objects():
            self.add(gtkobj)
        self.set_property('expand', True)
