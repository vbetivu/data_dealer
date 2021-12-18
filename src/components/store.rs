use std::fs;
use std::io::Write;
use std::{collections::HashMap, io::BufReader};

use super::component::{ComponentProps, ComponentType};

const STORE_FILE: &str = "store.json";

pub enum Action {
    SetQuery(String),
    AddNewEntry(String, String),
    RemoveEntry(String),
}

enum StoreAction {
    StateUpdate(Action),
    Subscribe(
        ComponentType,
        Box<dyn Fn(&State) -> ComponentProps + 'static>,
    ),
}

pub struct Subscriber {
    component: ComponentType,
    selector: Box<dyn Fn(&State) -> ComponentProps + 'static>,
}

impl Subscriber {
    fn new<S>(component: ComponentType, selector: S) -> Subscriber
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
    pub rows_by_id: HashMap<String, String>,
    pub query: String,
}

impl State {
    fn update(&mut self, action: Action) {
        match action {
            Action::SetQuery(payload) => self.set_query(payload),
            Action::AddNewEntry(key, value) => self.add(key, value),
            Action::RemoveEntry(key) => self.remove(key),
        }
    }

    fn add(&mut self, key: String, value: String) {
        self.rows_by_id.insert(key, value);

        self.sync_store_file();
    }

    fn set_query(&mut self, query: String) {
        self.query = query;
    }

    fn remove(&mut self, key: String) {
        self.rows_by_id.remove(&key);

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
        let mut store_file = fs::File::create(STORE_FILE).unwrap();

        store_file
            .write_all(serde_json::to_string(&self.rows_by_id).unwrap().as_bytes())
            .unwrap();
    }
}

pub struct Store {
    state: State,
    subscribers: Vec<Subscriber>,
}

impl Store {
    fn update(&mut self, action: StoreAction) {
        match action {
            StoreAction::StateUpdate(action) => self.state.update(action),
            StoreAction::Subscribe(component, selector) => self.subscribe(component, selector),
        }

        self.notify();
    }

    pub fn subscribe(
        &mut self,
        component: ComponentType,
        selector: Box<dyn Fn(&State) -> ComponentProps + 'static>,
    ) {
        let new_subscriber = Subscriber::new(component, selector);

        new_subscriber.notify(&self.state);

        self.subscribers.push(new_subscriber);
    }

    fn notify(&self) {
        for subscriber in &self.subscribers {
            subscriber.notify(&self.state);
        }
    }
}

pub struct Connect {
    dispatcher: glib::Sender<StoreAction>,
}

impl Connect {
    pub fn new() -> Connect {
        let store_file = fs::File::open(STORE_FILE).unwrap();

        let (tx, rx) = glib::MainContext::channel::<StoreAction>(glib::PRIORITY_DEFAULT);

        let state = State {
            rows_by_id: serde_json::from_reader(BufReader::new(&store_file)).unwrap(),
            query: String::new(),
        };

        let mut store = Store {
            state,
            subscribers: Vec::new(),
        };

        rx.attach(None, move |action| {
            store.update(action);

            return glib::Continue(true);
        });

        return Connect { dispatcher: tx };
    }

    pub fn subscribe(
        &self,
        component: ComponentType,
        selector: Box<dyn Fn(&State) -> ComponentProps + 'static>,
    ) {
        self.dispatcher
            .send(StoreAction::Subscribe(component, selector))
            .unwrap();
    }

    pub fn dispatch(&self, action: Action) {
        self.dispatcher
            .send(StoreAction::StateUpdate(action))
            .unwrap()
    }
}

impl Clone for Connect {
    fn clone(&self) -> Self {
        Connect {
            dispatcher: self.dispatcher.clone(),
        }
    }
}
