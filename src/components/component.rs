use super::list::{List as ListComponent, ListProps};

pub enum ComponentProps {
    List(ListProps),
}

pub enum ConnectComponentType {
    List(ListComponent),
}

impl ConnectComponentType {
    pub fn render(&self, props: ComponentProps) {
        match props {
            ComponentProps::List(props) => {
                if let ConnectComponentType::List(component) = self {
                    component.render(props)
                }
            }
        }
    }
}
