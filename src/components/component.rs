use super::list::{List as ListComponent, ListProps};

pub enum ComponentProps {
    List(ListProps),
}

pub enum ComponentType {
    List(ListComponent),
}

impl ComponentType {
    pub fn render(&self, props: ComponentProps) {
        match props {
            ComponentProps::List(props) => {
                let ComponentType::List(component) = self;

                component.render(props)
            }
        }
    }
}
