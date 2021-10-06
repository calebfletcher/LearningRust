use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn new(raw_headers: &str) -> Headers {
        let headers = raw_headers
            .split("\r\n")
            .map(Headers::decode_header)
            .map(|header| header.expect("Unable to decode header"));
        Headers {
            0: HashMap::from_iter(headers),
        }
    }

    fn decode_header(raw_header: &str) -> Option<(String, String)> {
        raw_header
            .split_once(": ")
            .map(|(key, value)| (String::from(key), String::from(value)))
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.0.get(&k[..])
    }
}
