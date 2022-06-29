use std::collections::HashMap;

pub struct Codec {
	pub index: usize,
    pub map: HashMap<String, String>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    pub fn new() -> Self {
        Codec {  index: 0, map: HashMap::new() }
    }
	
    // Encodes a URL to a shortened URL.
    pub fn encode(&mut self, longURL: String) -> String {
        let index_str = self.index.to_string();
        self.map.insert(index_str.clone(), longURL.clone());
        let res = "http://tinyurl.com/".to_string() + &index_str;
        self.index += 1;
        return res;
    }
	
    // Decodes a shortened URL to its original URL.
    pub fn decode(&self, shortURL: String) -> String {
        let strid = shortURL.replace("http://tinyurl.com/", "");
        let res = self.map.get(&strid).unwrap().clone();
        res
    }
}

