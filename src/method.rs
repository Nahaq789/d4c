#[derive(Debug, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE
}

impl Default for Method {
    fn default() -> Method {
        Method::GET
    }
}