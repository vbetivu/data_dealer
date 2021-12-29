use gtk::prelude::*;

use super::component::{ComponentProps, ComponentType};

use super::store::{Action, Connect, RowVariant, State};
use crate::utils::{add_child, add_children, write_to_clipboard, ClipboardValue};

pub type ListProps = Vec<RowVariant>;

pub struct ListContainer {
    pub component: gtk::Box,
}

impl ListContainer {
    pub fn new(connect: &Connect) -> ListContainer {
        let component = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let list = List::new(connect.clone());

        component.add(&list.root);

        connect.subscribe(
            ComponentType::List(list),
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

                result.sort_by(|a, b| get_row_variant_key(a).cmp(&get_row_variant_key(b)));

                let result = result
                    .into_iter()
                    .map(|e| match e {
                        RowVariant::Heading(x) => RowVariant::Heading(x.to_ascii_uppercase()),
                        _ => e,
                    })
                    .collect();

                return ComponentProps::List(result);
            }),
        );

        ListContainer { component }
    }
}

pub struct List {
    pub root: gtk::Box,
    dispatcher: Connect,
}

impl List {
    pub fn new(dispatcher: Connect) -> List {
        List {
            root: gtk::Box::new(gtk::Orientation::Vertical, 20),
            dispatcher,
        }
    }

    pub fn render(&self, props: ListProps) {
        let List { root, dispatcher } = self;

        root.children().iter().for_each(|child| root.remove(child));

        add_children(root, create_list(props, dispatcher));

        root.show_all();
    }
}

fn create_list(rows: Vec<RowVariant>, dispatcher: &Connect) -> Vec<gtk::Box> {
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

                let content = if value.starts_with("file::/") {
                    create_image_content(&key, value)
                } else {
                    create_text_content(&key, value)
                };

                let delete_button = gtk::Button::new();
                let delete_button_label = gtk::Label::new(Some("DEL"));

                delete_button.connect_clicked(move |_| {
                    dispatcher.dispatch(Action::RemoveEntry(String::from(&key)))
                });

                add_child(&section, &content);

                add_child(&section, add_child(&delete_button, &delete_button_label));

                sections.push(section);
            }
        }
    }

    return sections;
}

fn create_text_content(key: &String, value: String) -> gtk::Button {
    let content_button = gtk::Button::new();
    let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 8);

    add_child(&wrapper, &gtk::Label::new(Some(&key)));
    add_child(&wrapper, &gtk::Label::new(Some(&value)));

    content_button.connect_clicked(move |_| write_to_clipboard(ClipboardValue::Text(&value)));

    add_child(&content_button, &wrapper);

    return content_button;
}

fn create_image_content(key: &String, value: String) -> gtk::Button {
    let content_button = gtk::Button::new();
    let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 8);

    let pixbuf = gdk::gdk_pixbuf::Pixbuf::from_file(&value[7..]).unwrap();

    add_child(&wrapper, &gtk::Label::new(Some(&key)));
    add_child(&wrapper, &gtk::Image::from_pixbuf(Some(&pixbuf)));

    content_button.connect_clicked(move |_| write_to_clipboard(ClipboardValue::Image(&pixbuf)));

    add_child(&content_button, &wrapper);

    return content_button;
}

fn get_row_variant_key(value: &RowVariant) -> String {
    match value {
        RowVariant::Heading(x) => x.to_string().to_ascii_lowercase(),
        RowVariant::Data(x, _) => x.to_string().to_ascii_lowercase(),
    }
}
