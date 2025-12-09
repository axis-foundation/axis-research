use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    // literals
    Int(i64),
    Bool(bool),

    // variables
    Var(String),

    // lambda abstraction: fn x => body
    Lambda {
        param: String,
        #[serde(with = "serde_arc")]
        body: Arc<Expr>,
    },

    // application: f arg
    Apply {
        #[serde(with = "serde_arc")]
        func: Arc<Expr>,
        #[serde(with = "serde_arc")]
        arg: Arc<Expr>,
    },

    // let x = value in body
    Let {
        name: String,
        #[serde(with = "serde_arc")]
        value: Arc<Expr>,
        #[serde(with = "serde_arc")]
        body: Arc<Expr>,
    },

    // if cond then then_branch else else_branch
    If {
        #[serde(with = "serde_arc")]
        cond: Arc<Expr>,
        #[serde(with = "serde_arc")]
        then_branch: Arc<Expr>,
        #[serde(with = "serde_arc")]
        else_branch: Arc<Expr>,
    },

    // simple tuple and record for data
    Tuple(Vec<Expr>),
    Record(Vec<(String, Expr)>),
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
