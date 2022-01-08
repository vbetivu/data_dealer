use gtk::prelude::*;

use super::component::{ComponentProps, ComponentType};

use super::super::utils::add_child;
use super::store::{Action, Connect, EntryValue, State};

pub type AddButtonProps = bool;

pub struct AddButtonContainer {
    pub component: gtk::Box,
}

impl AddButtonContainer {
    pub fn new(connect: &Connect) -> AddButtonContainer {
        let component = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let button = AddButton::new(connect.clone());

        add_child(&component, &button.root);

        connect.subscribe(
            ComponentType::AddButton(button),
            Box::new(|state: &State| {
                let mut can_create = false;

                if state.query.len() != 0 && !state.rows_by_id.contains_key(&state.query) {
                    can_create = true;
                }

                return ComponentProps::AddButton(can_create);
            }),
        );

        AddButtonContainer { component }
    }
}

pub struct AddButton {
    pub root: gtk::Button,
}

impl AddButton {
    pub fn new(dispatcher: Connect) -> AddButton {
        let root = gtk::Button::new();
        let add_icon = gtk::Image::from_icon_name(Some("list-add"), gtk::IconSize::Button);

        add_child(&root, &add_icon);

        root.connect_clicked(move |_| {
            let text = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD).wait_for_text();
            let image = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD).wait_for_image();

            if let Some(text) = text {
                dispatcher.dispatch(Action::AddNewEntry(EntryValue::Text(text)));
            } else if let Some(image) = image {
                dispatcher.dispatch(Action::AddNewEntry(EntryValue::Image(image)))
            }
        });

        AddButton { root }
    }

    pub fn render(&self, props: AddButtonProps) {
        self.root.set_sensitive(props);
    }
}
