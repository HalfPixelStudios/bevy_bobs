//! Load prefabs from RON files

pub mod models;

use bevy::asset::AssetLoader;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::marker::PhantomData;

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

/*
struct PrefabLoader<T> {
    _marker: PhantomData<fn() -> T>
}

impl<T> AssetLoader for PrefabLoader<T>
where
    for<'de> T: Deserialize<'de> + Send + Sync + 'static
{
    fn load<'a>(
            &'a self,
            bytes: &'a [u8],
            load_context: &'a mut bevy::asset::LoadContext,
        ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
*/
