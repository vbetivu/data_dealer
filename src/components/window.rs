use gtk::prelude::*;
use gtk::{self, Widget};
use std::rc::Rc;

use super::store::{RowVariant, Store};

pub struct Window {
    window: gtk::ApplicationWindow,
    store: Rc<Store>,
}

impl Window {
    pub fn new(app: &gtk::Application, store: Store) -> Window {
        Window {
            window: gtk::ApplicationWindow::new(app),
            store: Rc::new(store),
        }
    }

    pub fn start(self) {
        let window = &self.window;

        self.set_end_conditions();
        self.set_window_properties();
        self.set_window_styles();
        self.render_children();

        window.show_all();
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
        // window.set_border_width(20);
    }

    fn render_children(&self) {
        let Window { window, store } = self;

        let main = gtk::Box::new(gtk::Orientation::Vertical, 24);

        let list = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        list.set_expand(true);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 20);

        let rows = store.get_rows();
        let mut sections: Vec<gtk::Box> = Vec::with_capacity(rows.len());

        for row in rows {
            match row {
                RowVariant::Heading(letter) => {
                    let section = gtk::Box::new(gtk::Orientation::Vertical, 16);
                    let label = gtk::Label::new(Some(&letter.to_string()));
                    section.add(&label);
                    sections.push(section);
                }
                RowVariant::Data(key, value) => {
                    let local_store = Rc::clone(store);

                    let section = gtk::Box::new(gtk::Orientation::Horizontal, 12);
                    let button = gtk::Button::new();
                    let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 8);
                    let key_label = gtk::Label::new(Some(&key));
                    let value_label = gtk::Label::new(Some(&value));

                    button.connect_clicked(move |_| write_to_clipboard(local_store.get(&key)));

                    add_child(
                        &section,
                        add_child(
                            &button,
                            add_children(&wrapper, vec![key_label, value_label]),
                        ),
                    );

                    sections.push(section);
                }
            }
        }

        add_child(
            window,
            add_child(&main, add_child(&list, add_children(&container, sections))),
        );
    }
}

fn add_child<'a, T: ContainerExt, U: IsA<Widget>>(element: &'a T, child: &U) -> &'a T {
    element.add(child);

    return element;
}

fn add_children<'a, T: ContainerExt, U: IsA<Widget>>(element: &'a T, children: Vec<U>) -> &'a T {
    for child in children {
        element.add(&child);
    }

    return element;
}

fn write_to_clipboard(value: &str) {
    let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);

    clipboard.set_text(value);
}
