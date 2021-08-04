use gtk::prelude::*;
use gtk::{self, Widget};
use std::cell::RefCell;
use std::rc::Rc;

use super::store::{RowVariant, Store};

pub struct Window {
    window: gtk::ApplicationWindow,
    store: Rc<RefCell<Store>>,
}

impl Window {
    pub fn new(app: &gtk::Application, store: Store) -> Window {
        Window {
            window: gtk::ApplicationWindow::new(app),
            store: Rc::new(RefCell::new(store)),
        }
    }

    pub fn start(self) {
        let ref window = self.window;

        self.set_end_conditions();
        self.set_window_properties();
        self.set_window_styles();
        self.render();

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

    fn render(&self) {
        let Window { window, store } = self;

        let main = gtk::Box::new(gtk::Orientation::Vertical, 24);

        let top_row = gtk::Box::new(gtk::Orientation::Horizontal, 12);

        let input = gtk::Entry::new();

        let add_button = gtk::Button::new();
        add_button.set_label("+");

        let list = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        list.set_expand(true);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 20);
        let container_holder = Rc::new(container);

        add_child(&top_row, &input);
        add_child(&top_row, &add_button);
        add_child(&main, &top_row);

        add_child(
            window,
            add_child(
                &main,
                add_child(
                    &list,
                    add_children(
                        container_holder.as_ref(),
                        create_list(store.borrow().get_rows("")),
                    ),
                ),
            ),
        );

        let input_store = Rc::clone(store);
        let input_container = Rc::clone(&container_holder);
        input.connect_changed(move |element| {
            input_container
                .children()
                .iter()
                .for_each(|child| input_container.remove(child));

            add_children(
                input_container.as_ref(),
                create_list(input_store.borrow().get_rows(&element.buffer().text())),
            );

            input_container.show_all();
        });

        let add_btn_store = Rc::clone(store);
        let button_container = Rc::clone(&container_holder);
        add_button.connect_clicked(glib::clone!(@weak window => move |_| {
            show_dialog(window, &add_btn_store);

            button_container
                .children()
                .iter()
                .for_each(|child| button_container.remove(child));

            add_children(
                button_container.as_ref(),
                create_list(add_btn_store.borrow().get_rows("")),
            );

            button_container.show_all();
        }));
    }
}

fn show_dialog<W: IsA<gtk::Window>>(window: W, store: &Rc<RefCell<Store>>) {
    let question_dialog = gtk::Dialog::new();

    question_dialog.set_transient_for(Some(&window));
    question_dialog.set_modal(true);
    question_dialog.set_decorated(false);
    question_dialog.set_destroy_with_parent(true);
    question_dialog.set_width_request(300);
    question_dialog.set_width_request(200);
    question_dialog.add_button("Cancel", gtk::ResponseType::Cancel);

    let ok_button = question_dialog.add_button("Create", gtk::ResponseType::Ok);

    ok_button.set_sensitive(false);

    let new_key = gtk::Entry::new();

    new_key.connect_changed(move |entry| {
        ok_button.set_sensitive(entry.text().len() != 0);
    });

    question_dialog
        .content_area()
        .pack_end(&new_key, true, true, 20);

    question_dialog.show_all();

    let result = question_dialog.run();

    match result {
        gtk::ResponseType::Ok => {
            let new_key = new_key.text().to_string();
            let new_value = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD)
                .wait_for_text()
                .unwrap()
                .to_string();

            store.borrow_mut().set(new_key, new_value)
        }
        _ => (),
    }

    question_dialog.close();

    unsafe {
        question_dialog.destroy();
    }
}

fn create_list(rows: Vec<RowVariant>) -> Vec<gtk::Box> {
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
                let section = gtk::Box::new(gtk::Orientation::Horizontal, 12);
                let button = gtk::Button::new();
                let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 8);
                let key_label = gtk::Label::new(Some(&key));
                let value_label = gtk::Label::new(Some(&value));

                button.connect_clicked(move |_| write_to_clipboard(&value));

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

    return sections;
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
