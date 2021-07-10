mod components;

use components::window::Window;
use gio::prelude::*;

fn on_activate(application: &gtk::Application) {
    let window = Window::new(application);

    window.start();
}

fn main() {
    let app = gtk::Application::new(Some("vbetivu.data-dealer"), Default::default());

    app.connect_activate(|app| on_activate(app));

    app.run();
}
