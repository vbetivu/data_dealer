use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use super::list::List;
use super::store::Store;
use crate::utils::add_child;

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
        let scrolling_section =
            gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let list = Rc::new(List::new());

        add_button.set_label("+");
        scrolling_section.set_expand(true);

        list.render(store.borrow().get_rows(""));

        add_child(&top_row, &input);
        add_child(&top_row, &add_button);
        add_child(&main, &top_row);

        add_child(
            window,
            add_child(&main, add_child(&scrolling_section, &list.root)),
        );

        let store_ref = Rc::clone(store);
        let list_ref = Rc::clone(&list);
        input.connect_changed(move |element| {
            list_ref.render(store_ref.borrow().get_rows(&element.buffer().text()));
        });

        let store_ref = Rc::clone(store);
        let list_ref = Rc::clone(&list);
        add_button.connect_clicked(glib::clone!(@weak window => move |_| {
            show_dialog(window, &store_ref);

            list_ref.render(store_ref.borrow().get_rows(""));
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
