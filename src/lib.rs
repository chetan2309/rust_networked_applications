use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore(HashMap<String, String>);

impl KvStore {
    pub fn new() -> KvStore {
        KvStore(HashMap::new())
    }

    pub fn set(&mut self, _key: String, _value: String) -> Option<String> {
        unimplemented!();
    }

    pub fn get(&mut self, _key: String) -> Option<String> {
        unimplemented!();
    }

    pub fn remove(&mut self, _key: String) -> Option<String> {
        unimplemented!();
    }
}