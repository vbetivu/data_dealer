use gtk::prelude::*;
use std::sync::mpsc::Sender;

use super::component::{ComponentProps, ConnectComponentType};

use super::store::{Action, RowVariant, State, Store};
use crate::utils::{add_child, add_children, write_to_clipboard};

pub type ListProps = Vec<RowVariant>;

pub struct ListContainer {
    pub component: gtk::Box,
}

impl ListContainer {
    pub fn new(store: glib::Sender<Action>) -> ListContainer {
        let component = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let list = List::new();

        component.add(&list.root);

        store
            .send(Action::Subscribe(
                ConnectComponentType::List(list),
                Box::new(|state: &State| {
                    let State {
                        rows_by_id, query, ..
                    } = state;

                    let query = query.trim();

                    let valid_map: Vec<(&String, &String)> = rows_by_id
                        .iter()
                        .filter(|(key, ..)| key.contains(query))
                        .collect();

                    let mut headings: Vec<char> = valid_map
                        .iter()
                        .map(|(key, ..)| key.chars().next().unwrap())
                        .collect();

                    headings.sort();
                    headings.dedup();

                    let mut result: Vec<RowVariant> = headings
                        .into_iter()
                        .map(|e| RowVariant::Heading(e))
                        .collect();

                    let mut data: Vec<RowVariant> = valid_map
                        .into_iter()
                        .map(|(key, value)| RowVariant::Data(key.clone(), value.clone()))
                        .collect();

                    result.append(&mut data);

                    result.sort_by(|a, b| get_compare_value(a).cmp(&get_compare_value(b)));

                    let result = result
                        .into_iter()
                        .map(|e| match e {
                            RowVariant::Heading(x) => RowVariant::Heading(x.to_ascii_uppercase()),
                            _ => e,
                        })
                        .collect();

                    return ComponentProps::List(result);
                }),
            ))
            .unwrap();

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

    pub fn render(&self, props: ListProps, dispatch: glib::Sender<Action>) {
        let List { root } = self;

        root.children().iter().for_each(|child| root.remove(child));

        add_children(root, create_list(props, dispatch));

        root.show_all();
    }
}

fn create_list(rows: Vec<RowVariant>, dispatcher: glib::Sender<Action>) -> Vec<gtk::Box> {
    let mut sections: Vec<gtk::Box> = Vec::with_capacity(rows.len());

    for row in rows {
        let dispatcher = dispatcher.clone();

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
                    dispatcher
                        .send(Action::RemoveEntry(String::from(&key)))
                        .unwrap()
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

fn get_compare_value(value: &RowVariant) -> String {
    match value {
        RowVariant::Heading(x) => x.to_string().to_ascii_lowercase(),
        RowVariant::Data(x, _) => x.to_string().to_ascii_lowercase(),
    }
}
