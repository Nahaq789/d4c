use std::fmt::Debug;

use crate::{header::Header, method::{self, Method}};

#[derive(Clone)]
pub struct Request<T> {
    parts: Parts,
    body: T,
}

#[derive(Clone)]
pub struct Parts {
    header: Header,
    uri: String,
    method: Method,
}

impl Parts {
    pub fn new() -> Parts {
        Parts {
            header: Header::default(),
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

    fn and_then<F>(self, func: F) -> Self
    where 
        F: FnOnce(Parts) -> Result<Parts>
    {
        Builder {
            inner: self.
        }
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
            header: Header::new(),
            uri: "some uri".to_string(),
            method: Method::GET,
        };
        let body = 0;
        let request = Request::new(parts, body);
        let a = Request::builder().method(Method::GET);
        println!("{:?}", a)
    }
}
