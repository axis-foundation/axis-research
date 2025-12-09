use crate::value::Value;
use im::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Env(#[serde(with = "serde_arc")] Arc<HashMap<String, Value>>);

impl Env {
    pub fn empty() -> Self {
        Env(Arc::new(HashMap::new()))
    }

    pub fn extend(&self, name: String, value: Value) -> Self {
        let mut new_map = (*self.0).clone();
        new_map.insert(name, value);
        Env(Arc::new(new_map))
    }

    pub fn lookup(&self, name: &str) -> Option<&Value> {
        self.0.get(name)
    }
}

// Helper module for serializing/deserializing Arc<T>
mod serde_arc {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::sync::Arc;

    pub fn serialize<S, T>(arc: &Arc<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        arc.as_ref().serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Arc<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        T::deserialize(deserializer).map(Arc::new)
    }
}
