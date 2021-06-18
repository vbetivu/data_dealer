extern crate gio;
extern crate gtk;

// To import all needed traits.
//use gio::prelude::*;
use gtk::{prelude::*, Window, WindowPosition, WindowType};

fn main() {
    gtk::init().unwrap();

    init_window();

    gtk::main();
}

fn init_window() {
    let popup = Window::new(WindowType::Popup);

    popup.set_size_request(500, 700);
    popup.set_position(WindowPosition::Center);

    popup.show_all();
}
