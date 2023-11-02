use std::collections::HashMap;

#[derive(Default)]
pub struct Attrs {
    values: HashMap<&'static str, String>,
    omit: Vec<&'static str>,
}
impl Attrs {
    pub fn omit(&self, fields_to_omit: Vec<&'static str>) -> Self {
        Self {
            values: self.values.clone(),
            omit: fields_to_omit,
        }
    }
    pub fn to_hashmap(&self) -> HashMap<&'static str, String> {
        let mut hashmap = self.values.clone();

        for field in &self.omit {
            hashmap.remove(field);
        }

        hashmap
    }
    pub fn get(&self, key: &'static str) -> Option<&String> {
        self.values.get(key)
    }
}

impl Clone for Attrs {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            omit: self.omit.clone(),
        }
    }
}

impl From<HashMap<&'static str, String>> for Attrs {
    fn from(html_attrs: HashMap<&'static str, String>) -> Self {
        Self {
            values: html_attrs,
            omit: vec![],
        }
    }
}
