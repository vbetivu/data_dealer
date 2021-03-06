mod components;
mod utils;

use gio::prelude::*;
use gtk::prelude::*;
use std::fs;

use components::store::Connect;
use components::window::Window;

fn build_ui(application: &gtk::Application, connect: Connect) {
    let window = Window::new(application, connect);

    window.start();
}

fn main() {
    let app = gtk::Application::new(Some("vbetivu.data-dealer"), Default::default());

    app.connect_activate(|app| {
        let connect = Connect::new();

        let provider = gtk::CssProvider::new();

        let style = fs::read("styles.css").unwrap();
        provider.load_from_data(&style).expect("Failed to load CSS");

        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        build_ui(&app, connect);
    });

    app.run();
}
