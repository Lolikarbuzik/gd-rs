use std::collections::HashMap;

pub fn parser(str: String, sep: char) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut curr_str = String::new();
    let mut values = Vec::new();

    for ch in str.chars() {
        if ch != sep {
            curr_str.push(ch);
        } else {
            values.push(curr_str.clone());
            curr_str.clear();
            if values.len() == 2 {
                let key = values.get(0).unwrap().clone();
                let value = values.get(1).unwrap().clone();
                map.insert(key, value);
                values.clear();
            }
        }
    }

    map
}
pub fn tags_replace(str: &mut String, tags: &[[&str; 2]], plus: &str) {
    for [f, t] in tags {
        *str = str.replacen(
            format!("{f}{plus}").as_str(),
            format!("{t}{plus}").as_str(),
            99999,
        );
    }
}

pub fn xor(b: &Vec<u8>, k: u8) -> Vec<u8> {
    b.clone().iter_mut().map(|v| *v ^ k).collect()
}
