use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Cookies(HashMap<String, String>);

impl Cookies {
    pub fn new(raw_cookies: &str) -> Option<Cookies> {
        let cookies_list: Vec<Option<(String, String)>> =
            raw_cookies.split(";").map(Cookies::decode_cookie).collect();

        if cookies_list.iter().any(|cookie| cookie.is_none()) {
            None
        } else {
            let cookies_list = cookies_list.into_iter().map(|cookie| cookie.unwrap());
            Some(Cookies {
                0: HashMap::from_iter(cookies_list),
            })
        }
    }

    fn decode_cookie(cookie: &str) -> Option<(String, String)> {
        let (key, value) = cookie.trim().split_once("=")?;
        Some((String::from(key), String::from(value)))
    }

    pub fn _get(&self, k: &str) -> Option<&String> {
        self.0.get(&k[..])
    }
}
