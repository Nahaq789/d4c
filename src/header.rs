

#[derive(Debug, Clone)]
pub struct Header<K: Default, V: Default> {
    content_type: K,
    value: V
}

impl Header {
    pub fn new() -> Self {
        Header::default()
    }
    pub fn kv<K, V>(mut self, k: K, v: V) -> Header {
        Header {
            content_type: k, value: v
        }
    }
    pub fn content_type(mut self, v: String) -> Header {
        self.content_type = v;
        Header::new()
    }
}

impl Default for Header {
    fn default() -> Self {
        Self { content_type: Default::default() }
    }
}