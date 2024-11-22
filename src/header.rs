use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Header {
    content_type: String,
    kv: HeaderMap
}

#[derive(Debug, Clone)]
pub struct HeaderMap {
    map: HashMap<String, String>
}

impl Header {
    pub fn new() -> Self {
        Header::default()
    }
    pub fn kv(mut self, k: String, v: String) -> Header {
        self.kv.map.insert(k, v);
        Header::new()
    }
    pub fn content_type(mut self, v: String) -> Header {
        self.content_type = v;
        Header::new()
    }
}

impl Default for Header {
    fn default() -> Self {
        Self { content_type: Default::default(), kv: HeaderMap::default() }
    }
}

impl Default for HeaderMap {
    fn default() -> Self {
        Self { map: Default::default() }
    }
}