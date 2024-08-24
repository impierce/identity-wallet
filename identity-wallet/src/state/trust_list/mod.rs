pub mod actions;
pub mod reducers;

use super::FeatTrait;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Not};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/trust_list/TrustLists.ts")]
pub struct TrustLists(pub Vec<TrustList>);

#[typetag::serde(name = "trust_lists")]
impl FeatTrait for TrustLists {}

impl Default for TrustLists {
    fn default() -> Self {
        let mut default = Self::new();
        default.insert(TrustList {
            name: "impierce".to_string(),
            trust_list: HashMap::from([("https://www.impierce.com".to_string(), true)]),
        });
        default
    }
}

impl TrustLists {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.0.iter().any(|trust_list| trust_list.name == name)
    }

    /// Modelled after the `std::collections::HashMap::insert` method.
    fn insert(&mut self, trust_list: TrustList) -> Option<&TrustList> {
        self.contains(&trust_list.name)
            .not()
            .then(|| {
                self.0.push(trust_list);
                self.0.last()
            })
            .flatten()
    }

    /// Modelled after the `std::collections::HashMap::get_mut` method.
    fn get_mut(&mut self, name: &str) -> Option<&mut TrustList> {
        self.0.iter_mut().find(|trust_list| trust_list.name == name)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS, PartialEq, Default)]
#[ts(export, export_to = "bindings/trust_list/TrustList.ts")]
#[serde(default)]
pub struct TrustList {
    name: String,
    trust_list: HashMap<String, bool>,
}

impl TrustList {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            trust_list: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, domain: String, trusted: bool) {
        self.trust_list.insert(domain, trusted);
    }

    pub fn remove(&mut self, domain: &str) {
        self.trust_list.remove(domain);
    }

    pub fn contains(&self, domain: &str) -> bool {
        self.trust_list.contains_key(domain)
    }

    pub fn get(&self, domain: &str) -> Option<&bool> {
        self.trust_list.get(domain)
    }

    pub fn get_mut(&mut self, domain: &str) -> Option<&mut bool> {
        self.trust_list.get_mut(domain)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, bool> {
        self.trust_list.iter()
    }
}
