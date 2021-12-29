use gtk::prelude::*;

pub enum ClipboardValue<'a> {
    Text(&'a str),
    Image(&'a gtk::gdk_pixbuf::Pixbuf),
}

pub fn add_child<'a, T: gtk::traits::ContainerExt, U: IsA<gtk::Widget>>(
    element: &'a T,
    child: &U,
) -> &'a T {
    element.add(child);

    return element;
}

pub fn add_children<'a, T: gtk::traits::ContainerExt, U: IsA<gtk::Widget>>(
    element: &'a T,
    children: Vec<U>,
) -> &'a T {
    for child in children {
        element.add(&child);
    }

    return element;
}

pub fn write_to_clipboard(value: ClipboardValue) {
    let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);

    match value {
        ClipboardValue::Image(image) => clipboard.set_image(image),
        ClipboardValue::Text(text) => clipboard.set_text(text),
    };
}
