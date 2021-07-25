mod components;

use gio::prelude::*;
use gtk::prelude::*;
use std::{fs, io::BufReader};

use components::store::Store;
use components::window::Window;

fn build_ui(application: &gtk::Application, store: Store) {
    let window = Window::new(application, store);

    window.start();
}

fn main() {
    let app = gtk::Application::new(Some("vbetivu.data-dealer"), Default::default());

    app.connect_activate(|app| {
        let store_file = fs::File::open("store.json").unwrap();
        let mut buf_reader = BufReader::new(store_file);
        let store = Store::new(&mut buf_reader);

        let provider = gtk::CssProvider::new();

        let style = fs::read("styles.css").unwrap();
        provider.load_from_data(&style).expect("Failed to load CSS");

        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        build_ui(&app, store);
    });

    app.run();
}
