use gtk;
use gtk::prelude::*;

pub struct Window(gtk::ApplicationWindow);

impl Window {
    pub fn new(app: &gtk::Application) -> Window {
        Window(gtk::ApplicationWindow::new(app))
    }

    pub fn start(self) {
        let Window(window) = &self;

        self.set_end_conditions();
        self.set_window_properties();
        self.set_window_styles();

        window.show_all();
    }

    fn set_end_conditions(&self) {
        let Window(window) = self;

        window.connect_focus_out_event(|window, _| {
            destroy(window);

            return Inhibit(false);
        });

        window.connect_key_press_event(|window, _| {
            destroy(window);

            return Inhibit(false);
        });
    }

    fn set_window_properties(&self) {
        let Window(window) = self;

        window.set_decorated(false);
        window.set_skip_taskbar_hint(true);
    }

    fn set_window_styles(&self) {
        let Window(window) = self;

        window.set_width_request(500);
        window.set_height_request(700);
        window.set_border_width(20);
    }
}

fn destroy(element: &gtk::ApplicationWindow) {
    unsafe {
        element.destroy();
    }
}
