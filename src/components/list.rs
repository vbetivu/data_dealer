use gtk::prelude::*;

use super::store::RowVariant;
use crate::utils::{add_child, add_children, write_to_clipboard};
pub struct List {
    pub root: gtk::Box,
}

impl List {
    pub fn new() -> List {
        List {
            root: gtk::Box::new(gtk::Orientation::Vertical, 20),
        }
    }

    pub fn render(&self, rows: Vec<RowVariant>) {
        let List { root } = self;

        root.children().iter().for_each(|child| root.remove(child));

        add_children(root, create_list(rows));

        root.show_all();
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
