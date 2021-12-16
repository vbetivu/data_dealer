use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, io::BufReader};

use super::component::{ComponentProps, ConnectComponentType};

const STORE_FILE: &str = "store.json";

pub enum Action {
    SetQuery(String),
    AddNewEntry(String, String),
    RemoveEntry(String),
    Subscribe(
        ConnectComponentType,
        Box<dyn Fn(&State) -> ComponentProps + 'static>,
    ),
}

pub struct Subscriber {
    component: ConnectComponentType,
    selector: Box<dyn Fn(&State) -> ComponentProps + 'static>,
}

impl Subscriber {
    fn new<S>(component: ConnectComponentType, selector: S) -> Subscriber
    where
        S: Fn(&State) -> ComponentProps + 'static,
    {
        Subscriber {
            component,
            selector: Box::new(selector),
        }
    }

    fn notify(&self, state: &State, dispatcher: glib::Sender<Action>) {
        let value = (self.selector)(state);

        self.component.render(value, dispatcher);
    }
}

pub enum RowVariant {
    Heading(char),
    Data(String, String),
}

pub struct State {
    rows_by_id: HashMap<String, String>,
    query: String,
}

impl State {
    pub fn get_visible_rows(&self) -> Vec<RowVariant> {
        let State {
            rows_by_id, query, ..
        } = &self;

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

        result.sort_by(|a, b| get_compare_value(a).cmp(&get_compare_value(b)));

        let result = result
            .into_iter()
            .map(|e| match e {
                RowVariant::Heading(x) => RowVariant::Heading(x.to_ascii_uppercase()),
                _ => e,
            })
            .collect();

        return result;
    }
}

pub struct Store {
    state: State,
    subscribers: Vec<Subscriber>,
    dispatcher: glib::Sender<Action>,
}

impl Store {
    pub fn new() -> glib::Sender<Action> {
        let store_file = fs::File::open(STORE_FILE).unwrap();

        // let (tx, rx): (Sender<Action>, Receiver<Action>) = mpsc::channel();

        let (tx, rx) = glib::MainContext::channel::<Action>(glib::PRIORITY_DEFAULT);

        let mut store = Store {
            state: State {
                rows_by_id: serde_json::from_reader(BufReader::new(&store_file)).unwrap(),
                query: String::new(),
            },
            subscribers: Vec::new(),
            dispatcher: tx.clone(),
        };

        rx.attach(None, move |action| {
            match action {
                Action::SetQuery(payload) => store.set_query(payload),
                Action::AddNewEntry(key, value) => store.add(key, value),
                Action::RemoveEntry(key) => store.remove(key),
                Action::Subscribe(component, selector) => store.subscribe(component, selector),
            }

            store.notify();

            return glib::Continue(true);
        });

        return tx;
    }

    pub fn subscribe(
        &mut self,
        component: ConnectComponentType,
        selector: Box<dyn Fn(&State) -> ComponentProps + 'static>,
    ) {
        let new_subscriber = Subscriber::new(component, selector);

        let dispatcher = self.dispatcher.clone();
        new_subscriber.notify(&self.state, dispatcher);

        self.subscribers.push(new_subscriber);
    }

    fn add(&mut self, key: String, value: String) {
        let Store { state, .. } = self;

        state.rows_by_id.insert(key, value);

        self.sync_store_file();
    }

    fn set_query(&mut self, query: String) {
        self.state.query = query;
    }

    fn remove(&mut self, key: String) {
        let Store { state, .. } = self;

        state.rows_by_id.remove(&key);

        self.sync_store_file();
    }

    fn notify(&self) {
        for subscriber in &self.subscribers {
            subscriber.notify(&self.state, self.dispatcher.clone());
        }
    }

    // pub fn get(&self, key: &str) -> &str {
    //     let Store { state, .. } = self;

    //     return state
    //         .rows_by_id
    //         .get(key)
    //         .expect(&format!("Missing key requested {}", key));
    // }

    fn sync_store_file(&self) {
        let Store { state, .. } = self;
        let mut store_file = fs::File::create(STORE_FILE).unwrap();

        store_file
            .write_all(serde_json::to_string(&state.rows_by_id).unwrap().as_bytes())
            .unwrap();
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }
}

fn get_compare_value(value: &RowVariant) -> String {
    match value {
        RowVariant::Heading(x) => x.to_string().to_ascii_lowercase(),
        RowVariant::Data(x, _) => x.to_string().to_ascii_lowercase(),
    }
}
