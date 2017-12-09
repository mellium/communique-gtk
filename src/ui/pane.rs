use gtk;

pub trait Pane<T: gtk::IsA<gtk::Widget>> {
    fn get_widget(self) -> T;
    fn get_builder(self) -> gtk::Builder;
}
