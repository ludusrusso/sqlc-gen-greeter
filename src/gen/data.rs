use convert_case::{Case, Casing};
use pluralizer::pluralize;

pub use plugin::*;

pub mod plugin {
    include!(concat!(env!("OUT_DIR"), "/plugin.rs"));
}

pub trait Named {
    fn name(&self) -> String;

    fn sing_snake(&self) -> String {
        pluralize(self.name().to_case(Case::Snake).as_str(), 1, false)
    }

    fn camel_case_name(&self) -> String {
        self.name().to_case(Case::UpperCamel)
    }
    fn plural(&self) -> String {
        pluralize(self.camel_case_name().as_str(), 2, false)
    }
    fn singular(&self) -> String {
        pluralize(self.camel_case_name().as_str(), 1, false)
    }
    fn ideomatic_go(&self) -> String {
        self.name()
            .to_case(Case::Snake)
            .split("_")
            .map(map_to_ideomatic_go)
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Named for plugin::Table {
    fn name(&self) -> String {
        self.rel.as_ref().unwrap().name.clone()
    }
}

impl Named for Column {
    fn name(&self) -> String {
        self.name.clone()
    }
}

fn map_to_ideomatic_go(t: &str) -> String {
    let gt = t.to_case(Case::UpperCamel);
    if gt == "Id" {
        return String::from("ID");
    }

    String::from(gt)
}
