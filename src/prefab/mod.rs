//! Load prefabs from RON files

pub mod models;

use std::{collections::HashMap, fs, path::Path};

use serde::de::DeserializeOwned;

/// Type used to index prefabs
pub type PrefabId = String;

type PrefabMap<P> = HashMap<PrefabId, P>;

/// Resource to store prefabs
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
