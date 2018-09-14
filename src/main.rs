extern crate rand;
mod ketama;

use rand::distributions::{Alphanumeric, Distribution};
use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("./test.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    let mut server_list: Vec<String> = Vec::new();

    while let Ok(i) = buf_reader.read_line(&mut line) {
        if i > 0 {
            if line.starts_with("#") {
                continue;
            }
            let srv = line.trim().to_owned();
            server_list.push(srv);
            line.clear();
        } else {
            break;
        }
    }

    let mut key_set: Vec<String> = Vec::new();
    let mut srv_map: HashMap<String, usize> = HashMap::new();
    let mut t_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut rng = thread_rng();

    let c = ketama::ketama_roll(&server_list, ketama::HashFunction::FNV1a).unwrap();

    for _ in 1..1000 {
        let k: String = Alphanumeric.sample_iter(&mut rng).take(7).collect();
        key_set.push(k);
    }
    key_set.dedup();

    for key in key_set.iter() {
        let s = ketama::ketama_get_server(&c, &key);
        srv_map.entry(s).and_modify(|v| *v += 1).or_insert(1);
    }

    // stats
    let mut vals: Vec<usize> = srv_map.values().map(|v| v.clone()).collect();
    vals.sort();
    let avg: usize = vals.iter().sum::<usize>() / vals.len();
    let high = vals.last().unwrap();
    let low = vals.first().unwrap();

    // verifying _consistent_ hashing i.e. a single key is ever mapped to only
    // one server
    for _ in 1..2000 {
        let r = rng.gen_range(0, key_set.len());
        let rkey = key_set.get(r).unwrap().to_owned();
        let srv = ketama::ketama_get_server(&c, &rkey);

        // This datastructure represents a random selection of keys that are
        // mapped to a server - we are testing to verify that a single key
        // will always map to only a single server, thus
        // key -> Set(servers...) where Set should be literally one entry.
        // clone() is used below b/c the borrow checker doesn't understand
        // that this code can only execute in one branch, never both.
        t_map
            .entry(rkey)
            .and_modify(|v| {
                v.insert(srv.clone());
            })
            .or_insert({
                let mut m: HashSet<String> = HashSet::new();
                m.insert(srv.clone());
                m
            });
    }

    let mut verify = true;
    for (k, v) in t_map.iter() {
        if v.len() > 1 {
            println!("Multi-Mapped Key: {} -> {:#?}", k, v);
            verify = false;
        }
    }
    // just a friendly message
    if verify {
        println!("No Multi-Mapped Keys found.");
    }

    println!("{:#?}", srv_map);
    println!("Keys: {}", key_set.len());
    println!("Avg: {}", avg);
    println!("High: {}", high);
    println!("Low: {}", low);
    println!("Distance: {}", high - low);
}
