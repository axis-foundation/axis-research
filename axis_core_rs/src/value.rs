use crate::ast::Expr;
use crate::env::Env;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Bool(bool),

    Tuple(Vec<Value>),
    Record(Vec<(String, Value)>),

    // Closure closes over an immutable Env
    Closure {
        param: String,
        #[serde(with = "serde_arc")]
        body: Arc<Expr>,
        env: Env,
    },
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
