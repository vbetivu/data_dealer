use gtk::prelude::*;

use super::component::{ComponentProps, ConnectComponentType};

use super::store::{RowVariant, State, Store};
use crate::utils::{add_child, add_children, write_to_clipboard};

pub type ListProps = Vec<RowVariant>;

pub struct ListContainer {
    pub component: gtk::Box,
}

impl ListContainer {
    pub fn new(store: &mut Store) -> ListContainer {
        let component = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let list = List::new();

        component.add(&list.root);

        store.subscribe(
            |state: &State| return ComponentProps::List(state.get_visible_rows()),
            ConnectComponentType::List(list),
        );

        ListContainer { component }
    }
}

pub struct List {
    pub root: gtk::Box,
}

impl List {
    pub fn new() -> List {
        List {
            root: gtk::Box::new(gtk::Orientation::Vertical, 20),
        }
    }

    pub fn render(&self, props: ListProps) {
        let List { root } = self;

        root.children().iter().for_each(|child| root.remove(child));

        add_children(root, create_list(props));

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
                let content_button = gtk::Button::new();
                let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 8);
                let key_label = gtk::Label::new(Some(&key));
                let value_label = gtk::Label::new(Some(&value));
                let delete_button = gtk::Button::new();
                let delete_button_label = gtk::Label::new(Some("DEL"));

                content_button.connect_clicked(move |_| write_to_clipboard(&value));

                delete_button.connect_clicked(move |_| {
                    println!("delete");
                });

                add_child(
                    &section,
                    add_child(
                        &content_button,
                        add_children(&wrapper, vec![key_label, value_label]),
                    ),
                );

                add_child(&section, add_child(&delete_button, &delete_button_label));

                sections.push(section);
            }
        }
    }

    return sections;
}
