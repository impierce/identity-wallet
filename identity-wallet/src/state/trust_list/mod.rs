pub mod actions;
pub mod reducers;

use super::FeatTrait;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Not};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, TS, PartialEq, Default)]
#[ts(export, export_to = "bindings/trust_list/TrustLists.ts")]
pub struct TrustLists(pub Vec<TrustList>);

#[typetag::serde(name = "trust_lists")]
impl FeatTrait for TrustLists {}

impl TrustLists {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn contains(&self, id: &str) -> bool {
        self.0.iter().any(|trust_list| trust_list.id == id)
    }

    /// Modelled after the `std::collections::HashMap::insert` method.
    pub fn insert(&mut self, trust_list: TrustList) -> Option<&TrustList> {
        self.contains(&trust_list.id)
            .not()
            .then(|| {
                self.0.push(trust_list);
                self.0.last()
            })
            .flatten()
    }

    /// Modelled after the `std::collections::HashMap::get_mut` method.
    fn get_mut(&mut self, id: &str) -> Option<&mut TrustList> {
        self.0.iter_mut().find(|trust_list| trust_list.id == id)
    }

    fn remove(&mut self, id: &str) -> Option<TrustList> {
        let index = self.0.iter().position(|trust_list| trust_list.id == id)?;
        Some(self.0.remove(index))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/trust_list/TrustList.ts")]
pub struct TrustList {
    pub id: String,
    pub display_name: String,
    pub custom: bool,
    pub entries: HashMap<String, bool>,
}

impl Default for TrustList {
    fn default() -> Self {
        Self::new()
    }
}

impl TrustList {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            display_name: String::new(),
            custom: true,
            entries: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, domain: String, trusted: bool) {
        self.entries.insert(domain, trusted);
    }

    pub fn remove(&mut self, domain: &str) {
        self.entries.remove(domain);
    }

    pub fn contains(&self, domain: &str) -> bool {
        self.entries.contains_key(domain)
    }

    pub fn get(&self, domain: &str) -> Option<&bool> {
        self.entries.get(domain)
    }

    pub fn get_mut(&mut self, domain: &str) -> Option<&mut bool> {
        self.entries.get_mut(domain)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, bool> {
        self.entries.iter()
    }
}
