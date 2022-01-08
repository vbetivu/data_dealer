use gtk::prelude::*;
use pango;

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

        component.set_expand(true);

        let list = List::new(connect.clone());

        add_child(&component, &list.root);

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
        let root = gtk::Box::new(gtk::Orientation::Vertical, 8);

        root.set_expand(true);

        List { root, dispatcher }
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
                let section = gtk::Box::new(gtk::Orientation::Horizontal, 0);

                section.set_halign(gtk::Align::Start);

                let letter_wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);

                letter_wrapper.set_widget_name("letter_wrapper");

                let label = gtk::Label::new(Some(&letter.to_string()));

                label.set_widget_name("letter_label");

                add_child(&letter_wrapper, &label);
                add_child(&section, &letter_wrapper);

                let bookmark = gtk::Box::new(gtk::Orientation::Vertical, 0);

                bookmark.set_widget_name("bookmark");

                add_child(&section, &bookmark);

                sections.push(section);
            }
            RowVariant::Data(key, value) => {
                let section = gtk::Box::new(gtk::Orientation::Horizontal, 12);

                section.set_widget_name("data_row");
                section.set_vexpand(false);

                let content = if value.starts_with("file::/") {
                    create_image_content(&key, value)
                } else {
                    create_text_content(&key, value)
                };

                let delete_button = gtk::Button::new();
                let delete_icon = gtk::Image::from_icon_name(Some("delete"), gtk::IconSize::Button);

                delete_button.connect_clicked(move |_| {
                    dispatcher.dispatch(Action::RemoveEntry(String::from(&key)))
                });

                delete_button.set_valign(gtk::Align::Center);

                add_child(&section, &content);

                add_child(&section, add_child(&delete_button, &delete_icon));

                sections.push(section);
            }
        }
    }

    return sections;
}

fn create_text_content(key: &String, value: String) -> gtk::Button {
    let content_button = gtk::Button::new();
    content_button.set_widget_name("content_button");
    content_button.set_expand(true);

    let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 8);

    wrapper.set_homogeneous(true);

    let key_label = gtk::Label::new(Some(&key));

    key_label.set_halign(gtk::Align::Start);
    key_label.set_ellipsize(pango::EllipsizeMode::End);
    key_label.set_single_line_mode(true);

    let value_label = gtk::Label::new(Some(&value));

    value_label.set_halign(gtk::Align::Start);
    value_label.set_ellipsize(pango::EllipsizeMode::Middle);
    value_label.set_single_line_mode(true);

    add_child(&wrapper, &key_label);
    add_child(&wrapper, &value_label);

    content_button.connect_clicked(move |_| write_to_clipboard(ClipboardValue::Text(&value)));

    add_child(&content_button, &wrapper);

    return content_button;
}

fn create_image_content(key: &String, value: String) -> gtk::Button {
    let content_button = gtk::Button::new();
    content_button.set_widget_name("content_button");
    content_button.set_expand(true);

    let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 8);

    wrapper.set_homogeneous(true);

    let key_label = gtk::Label::new(Some(&key));

    key_label.set_halign(gtk::Align::Start);

    let mut pixbuf = gdk::gdk_pixbuf::Pixbuf::from_file(&value[7..]).unwrap();

    pixbuf = pixbuf
        .scale_simple(150, 150, gdk::gdk_pixbuf::InterpType::Bilinear)
        .unwrap();

    let value_image = gtk::Image::from_pixbuf(Some(&pixbuf));

    add_child(&wrapper, &key_label);
    add_child(&wrapper, &value_image);

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
