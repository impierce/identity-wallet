pub mod actions;
pub mod reducers;

use super::FeatTrait;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Not};
use ts_rs::TS;
use url::Url;
use uuid::Uuid;

/// TrustLists is a Vec capable of holding TrustList's of 2 types, external and custom.
/// External currently would mean injected at profile creation just like our default trust list.
/// Custom TrustList's can be created in dev mode at any time.
/// A TrustList will contain trusted domains and/or URL's.
/// These will determine whether a Linked VP is to be trusted and therefore displayed to the user or not.
/// A default trust list has been added as well, containing domains we use in our demos.
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

    /// Modelled after the `std::collections::HashMap::remove` method.
    fn remove(&mut self, id: &str) -> Option<TrustList> {
        let index = self.0.iter().position(|trust_list| trust_list.id == id)?;
        Some(self.0.remove(index))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/trust_list/TrustList.ts")]
pub struct TrustList {
    #[serde(default)]
    pub id: String,
    pub display_name: String,
    #[serde(default)]
    pub custom: bool,
    #[serde(alias = "domains", deserialize_with = "deserialize_domains")]
    #[ts(type = "Record<string, boolean>")]
    pub entries: HashMap<url::Url, bool>,
}

fn deserialize_domains<'de, D>(deserializer: D) -> Result<HashMap<Url, bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let domains: Vec<Url> = Vec::deserialize(deserializer)?;
    Ok(domains.into_iter().map(|domain| (domain, true)).collect())
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

    pub fn insert(&mut self, domain: Url, trusted: bool) {
        self.entries.insert(domain, trusted);
    }

    pub fn remove(&mut self, domain: &Url) {
        self.entries.remove(domain);
    }

    pub fn contains(&self, domain: &Url) -> bool {
        self.entries.contains_key(domain)
    }

    pub fn get(&self, domain: &Url) -> Option<&bool> {
        self.entries.get(domain)
    }

    pub fn get_mut(&mut self, domain: &Url) -> Option<&mut bool> {
        self.entries.get_mut(domain)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<Url, bool> {
        self.entries.iter()
    }
}
