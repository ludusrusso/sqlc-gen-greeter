use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_tables")]
    pub tables: Vec<TableConfig>,
}

fn default_tables() -> Vec<TableConfig> {
    vec![]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableConfig {
    pub(crate) table: String,
    #[serde(default = "default_id_cols")]
    pub(crate) id_cols: Vec<String>,

    #[serde(default = "default_tenants_cols")]
    pub(crate) tenants_cols: Vec<String>,
}

fn default_id_cols() -> Vec<String> {
    vec!["id".to_string()]
}

fn default_tenants_cols() -> Vec<String> {
    vec![]
}

impl TableConfig {
    pub fn new(table: &str) -> Self {
        TableConfig {
            table: table.to_string(),
            ..Default::default()
        }
    }
}

impl Default for TableConfig {
    fn default() -> Self {
        TableConfig {
            table: "".to_string(),
            id_cols: default_id_cols(),
            tenants_cols: default_tenants_cols(),
        }
    }
}
