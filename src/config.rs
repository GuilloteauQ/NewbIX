use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preference {
    pub name: String,
    pub packages: Vec<String>,
}
