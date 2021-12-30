use super::add_button::{AddButton as AddButtonComponent, AddButtonProps};
use super::list::{List as ListComponent, ListProps};

pub enum ComponentProps {
    List(ListProps),
    AddButton(AddButtonProps),
}

pub enum ComponentType {
    List(ListComponent),
    AddButton(AddButtonComponent),
}

impl ComponentType {
    pub fn render(&self, props: ComponentProps) {
        match props {
            ComponentProps::List(props) => {
                if let ComponentType::List(component) = self {
                    component.render(props)
                }
            }
            ComponentProps::AddButton(props) => {
                if let ComponentType::AddButton(component) = self {
                    component.render(props)
                }
            }
        }
    }
}
