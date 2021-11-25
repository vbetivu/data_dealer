use std::fs;
use std::io::Write;
use std::{collections::HashMap, io::BufReader};

use super::component::{ComponentProps, ConnectComponentType};

const STORE_FILE: &str = "store.json";

pub enum Action<'a> {
    SetQuery(String),
    AddNewEntry(String, String),
    RemoveEntry(&'a str),
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

    fn notify(&self, state: &State) {
        let value = (self.selector)(state);

        self.component.render(value);
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
}

impl Store {
    pub fn new() -> Store {
        let store_file = fs::File::open(STORE_FILE).unwrap();

        return Store {
            state: State {
                rows_by_id: serde_json::from_reader(BufReader::new(&store_file)).unwrap(),
                query: String::new(),
            },
            subscribers: Vec::new(),
        };
    }

    pub fn dispatch(&mut self, action: Action) {
        match action {
            Action::SetQuery(payload) => self.set_query(payload),
            Action::AddNewEntry(key, value) => self.add(key, value),
            Action::RemoveEntry(key) => self.remove(key),
        }

        self.notify();
    }

    pub fn subscribe<S>(&mut self, component: ConnectComponentType, selector: S)
    where
        S: Fn(&State) -> ComponentProps + 'static,
    {
        let Store {
            subscribers, state, ..
        } = self;

        let new_subscriber = Subscriber::new(component, selector);

        new_subscriber.notify(state);

        subscribers.push(new_subscriber);
    }

    fn add(&mut self, key: String, value: String) {
        let Store { state, .. } = self;

        state.rows_by_id.insert(key, value);

        self.sync_store_file();
    }

    fn set_query(&mut self, query: String) {
        self.state.query = query;
    }

    fn notify(&self) {
        let Store {
            subscribers, state, ..
        } = self;

        for subscriber in subscribers {
            subscriber.notify(state);
        }
    }

    fn remove(&mut self, key: &str) {
        let Store { state, .. } = self;

        state.rows_by_id.remove(key);

        self.sync_store_file();
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
}

fn get_compare_value(value: &RowVariant) -> String {
    match value {
        RowVariant::Heading(x) => x.to_string().to_ascii_lowercase(),
        RowVariant::Data(x, _) => x.to_string().to_ascii_lowercase(),
    }
}
