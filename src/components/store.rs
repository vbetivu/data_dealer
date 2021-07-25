use std::{collections::HashMap, io::Read};

pub enum RowVariant {
    Heading(char),
    Data(String, String),
}

pub struct Store(HashMap<String, String>);

impl Store {
    pub fn new(reader: &mut dyn Read) -> Store {
        Store(serde_json::from_reader(reader).unwrap())
    }

    pub fn get(&self, key: &str) -> &str {
        let Store(map) = self;

        map.get(key)
            .expect(&format!("Missing key requested {}", key))
    }

    pub fn get_rows(&self, query: &str) -> Vec<RowVariant> {
        let Store(store) = self;

        let query = query.trim();

        let valid_map: Vec<(&String, &String)> = store
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

fn get_compare_value(value: &RowVariant) -> String {
    match value {
        RowVariant::Heading(x) => x.to_string().to_ascii_lowercase(),
        RowVariant::Data(x, _) => x.to_string().to_ascii_lowercase(),
    }
}
