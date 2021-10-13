use std::collections::HashMap;

#[derive(Debug)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn new(raw_headers: &str) -> Self {
        let headers = raw_headers
            .split("\r\n")
            .map(Self::decode_header)
            .map(|header| header.expect("Unable to decode header"));
        Self {
            0: headers.collect(),
        }
    }

    fn decode_header(raw_header: &str) -> Option<(String, String)> {
        raw_header
            .split_once(": ")
            .map(|(key, value)| (String::from(key), String::from(value)))
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.0.get(k)
    }
}
