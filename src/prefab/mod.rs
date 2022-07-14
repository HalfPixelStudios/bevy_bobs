//! Load prefabs from RON files

pub mod models;

use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub type PrefabId = String;

type PrefabMap<P> = HashMap<PrefabId, P>;

pub struct PrefabLib<P: DeserializeOwned> {
    map: PrefabMap<P>,
}

impl<P: DeserializeOwned> PrefabLib<P> {
    pub fn new(ron_string: &str) -> Self {
        let map: PrefabMap<P> = ron::from_str(ron_string).unwrap();

        PrefabLib { map }
    }
    pub fn from_file(filepath: &str) -> Self {
        let ron_string = fs::read_to_string(Path::new(&filepath)).unwrap();
        PrefabLib::<P>::new(&ron_string)
    }

    pub fn get(&self, id: &str) -> Option<&P> {
        self.map.get(id)
    }
}
