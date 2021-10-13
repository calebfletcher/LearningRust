use std::collections::HashMap;

#[derive(Debug)]
pub struct Cookies(HashMap<String, String>);

impl Cookies {
    pub fn new(raw_cookies: &str) -> Self {
        let cookies_list: Vec<Option<(String, String)>> =
            raw_cookies.split(';').map(Self::decode_cookie).collect();

        if cookies_list.iter().any(Option::is_none) {
            Self { 0: HashMap::new() }
        } else {
            let cookies_list = cookies_list.into_iter().map(Option::unwrap);
            Self {
                0: cookies_list.collect(),
            }
        }
    }

    fn decode_cookie(cookie: &str) -> Option<(String, String)> {
        let (key, value) = cookie.trim().split_once("=")?;
        Some((String::from(key), String::from(value)))
    }

    pub fn _get(&self, k: &str) -> Option<&String> {
        self.0.get(k)
    }
}
