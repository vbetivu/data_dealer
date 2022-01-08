use gtk::prelude::*;

use super::add_button::AddButtonContainer;
use super::list::ListContainer;
use super::store::{Action, Connect};
use crate::utils::add_child;

pub struct Window {
    window: gtk::ApplicationWindow,
    connect: Connect,
}

impl Window {
    pub fn new(app: &gtk::Application, connect: Connect) -> Window {
        Window {
            window: gtk::ApplicationWindow::new(app),
            connect,
        }
    }

    pub fn start(self) {
        self.set_end_conditions();
        self.set_window_properties();
        self.set_window_styles();
        self.render();
        self.window.show_all();
    }

    fn set_end_conditions(&self) {
        let window = &self.window;

        window.connect_focus_out_event(|window, _| {
            window.close();

            return Inhibit(false);
        });

        window.connect_key_press_event(|window, event| {
            if event.keyval() == gdk::keys::constants::Escape {
                window.close();
            }

            return Inhibit(false);
        });
    }

    fn set_window_properties(&self) {
        let window = &self.window;

        window.set_decorated(false);
        window.set_skip_taskbar_hint(true);
    }

    fn set_window_styles(&self) {
        let window = &self.window;

        window.set_width_request(500);
        window.set_height_request(700);
        window.set_border_width(20);
    }

    fn render(&self) {
        let main = gtk::Box::new(gtk::Orientation::Vertical, 16);
        let top_row = gtk::Box::new(gtk::Orientation::Horizontal, 10);

        top_row.set_widget_name("top_row");

        let input = gtk::SearchEntry::new();

        input.set_hexpand(true);

        let add_button = AddButtonContainer::new(&self.connect);

        let horizontal_line = gtk::Separator::new(gtk::Orientation::Horizontal);

        let scrolling_section =
            gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);

        scrolling_section.set_expand(true);

        let list = ListContainer::new(&self.connect);

        add_child(&top_row, &input);
        add_child(&top_row, &add_button.component);
        add_child(&main, &top_row);
        add_child(&main, &horizontal_line);

        add_child(
            &self.window,
            add_child(&main, add_child(&scrolling_section, &list.component)),
        );

        let cloned = self.connect.clone();

        input.connect_changed(move |element| {
            cloned.dispatch(Action::SetQuery(element.buffer().text()));
        });
    }
}
