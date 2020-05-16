use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn extract_namelist(path: &[String]) -> Vec<String> {
    let mut res = vec![];
    for p in path.iter() {
        let file = File::open(p).unwrap();
        for line in BufReader::new(file).lines() {
            if let Ok(content) = line {
                if content.contains("open") {
                    if let Some(name) = content.split(' ').nth(2) {
                        res.push(name.to_string());
                    }
                }
            }
        }
    }
    res
}

pub fn build_abbreviation_map(namelist: &[String]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut conflict: HashSet<String> = HashSet::new();
    for name in namelist.iter() {
        let tokens = name.split(':').collect::<Vec<&str>>();
        // the last entry
        if let Some(key) = tokens.last() {
            let lkey = key.to_string().to_lowercase();
            if map.contains_key(&lkey) | conflict.contains(&lkey) {
                map.remove(&lkey);
                conflict.insert(lkey);
            } else {
                map.insert(lkey, name.to_string());
            }
        }
        // the first letters
        let abbr = tokens
            .iter()
            .map(|e| e.chars().next().unwrap())
            .collect::<String>()
            .to_lowercase();
        if map.contains_key(&abbr.to_string()) | conflict.contains(&abbr.to_string()) {
            map.remove(&abbr);
        } else {
            map.insert(abbr, name.to_string());
        }
        map.insert(name.to_lowercase(), name.to_string());
    }
    map
}
