#[derive(Clone)]
pub struct Request<T> {
    parts: Parts,
    body: T
}

#[derive(Debug, Default, Clone)]
pub struct Parts {
    header: String,
    uri: String,
    method: String
}

#[derive(Debug, Default)]
pub struct Builder {
    inner: Parts
}

impl Request<()> {
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl <T> Request<T> {
    fn new(parts: Parts, body: T) -> Self {
        Self { parts, body }
    }
}

impl Builder {
    #[inline]
    pub fn new() -> Builder {
        Builder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_header() {
        let parts = Parts {
            header: "some header".to_string(),
            uri: "some uri".to_string(),
            method: "GET".to_string()
        };
        let body = 0;
        let request = Request::new(parts, body);
    }
}