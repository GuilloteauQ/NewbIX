use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preference {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub interpretor: String,
    #[serde(default)]
    #[serde(alias = "shellHook")]
    pub shell_hook: String,
    pub packages: Vec<String>,
}
