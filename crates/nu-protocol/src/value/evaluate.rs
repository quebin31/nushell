use crate::value::{Primitive, UntaggedValue, Value};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// An evaluation scope. Scopes map variable names to Values and aid in evaluating blocks and expressions.
/// Additionally, holds the value for the special $it variable, a variable used to refer to the value passing
/// through the pipeline at that moment
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Scope {
    pub it: Value,
    pub vars: IndexMap<String, Value>,
}

impl Scope {
    /// Create a new scope
    pub fn new(it: Value) -> Scope {
        Scope {
            it,
            vars: IndexMap::new(),
        }
    }
}

impl Scope {
    /// Create an empty scope
    pub fn empty() -> Scope {
        Scope {
            it: UntaggedValue::Primitive(Primitive::Nothing).into_untagged_value(),
            vars: IndexMap::new(),
        }
    }

    /// Create an empty scope, setting $it to a known Value
    pub fn it_value(value: Value) -> Scope {
        Scope {
            it: value,
            vars: IndexMap::new(),
        }
    }

    pub fn set_it(self, value: Value) -> Scope {
        Scope {
            it: value,
            vars: self.vars,
        }
    }

    pub fn set_var(self, name: String, value: Value) -> Scope {
        let mut new_vars = self.vars.clone();
        new_vars.insert(name, value);
        Scope {
            it: self.it,
            vars: new_vars,
        }
    }
}
