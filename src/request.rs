use std::{collections::HashMap, fmt::Debug, vec};

use crate::{errors, header::{HeaderKey, HeaderValue}, method::Method};

#[derive(Clone)]
pub struct Request<T> {
    parts: Parts,
    body: T,
}

#[derive(Clone)]
pub struct Parts {
    header: HashMap<HeaderKey, HeaderValue>,
    uri: String,
    method: Method,
}

impl Parts {
    pub fn new() -> Parts {
        Parts {
            header: HashMap::new(),
            uri: "hoge".to_string(),
            method: Method::default(),
        }
    }
}

impl Debug for Parts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parts")
            .field("header", &self.header)
            .field("uri", &self.uri)
            .field("method", &self.method)
            .finish()
    }
}

#[derive(Debug)]
pub struct Builder {
    inner: Parts,
}

impl Request<()> {
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl<T> Request<T> {
    fn new(parts: Parts, body: T) -> Self {
        Self { parts, body }
    }
}

impl Builder {
    #[inline]
    pub fn new() -> Builder {
        Builder::default()
    }

    pub fn method(&mut self, method: Method) -> Builder {
        self.inner.method = method;
        Builder::new()
    }

    pub fn header<K, V>(&mut self, key: K, value: V) -> Builder {
        self.inner.header.kv(key, value)
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Builder {
        Builder {
            inner: Parts::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        
        let parts = Parts {
            header: vec![],
            uri: "some uri".to_string(),
            method: Method::GET,
        };
        let body = 0;
        let request = Request::new(parts, body);
        let a = Request::builder().method(Method::GET);
        println!("{:?}", a)
    }
}
