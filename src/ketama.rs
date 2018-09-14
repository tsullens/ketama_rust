extern crate rand;

use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;

use std::sync::RwLock;

pub enum HashFunction {
    OneAtATime,
    FNV1a,
}

pub struct Continuum {
    //server_set: HashSet<String>,
    ring: RwLock<Vec<RingNode>>,
    hashfunc: fn(&[u8]) -> u32,
}

#[derive(Debug)]
struct RingNode {
    i: u32,
    srv: String,
}

impl PartialEq for RingNode {
    fn eq(&self, other: &RingNode) -> bool {
        self.i == other.i
    }
}

impl Eq for RingNode {}

impl PartialOrd for RingNode {
    fn partial_cmp(&self, other: &RingNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RingNode {
    fn cmp(&self, other: &RingNode) -> Ordering {
        if self.i < other.i {
            Ordering::Less
        } else if self.i > other.i {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Continuum {
    fn find_server(&self, val: u32) -> String {
        let handle = self.ring.read().unwrap();

        // quick check for our outside bounds
        if val > handle.last().unwrap().i || val < handle.first().unwrap().i {
            return handle.first().unwrap().srv.to_owned();
        }
        // recursively search
        match Continuum::_search(handle.as_slice(), val) {
            Some(ref node) => node.srv.to_owned(),
            None => "Something fucked up".to_owned(),
        }
    }

    fn _search(slc: &[RingNode], val: u32) -> Option<&RingNode> {
        // If we have recursed down to 10 elements,
        // then let's just quickly search them instead of splitting
        if slc.len() <= 10 {
            let mut iter = slc.iter().skip_while(|node| val > node.i);
            // will return Some(node) or None
            return iter.next();
        } else {
            // it just werks
            let mid = slc.len() / 2;
            if val > slc[mid].i {
                // Search lower
                return Continuum::_search(&slc[mid + 1..], val);
            } else {
                // Search uper
                return Continuum::_search(&slc[..mid + 1], val);
            }
        }
    }
}

pub fn ketama_roll(servers: &[String], hashfunc: HashFunction) -> Result<Continuum, Box<Error>> {
    let mut ring: Vec<RingNode> = Vec::new();
    let mut rng = thread_rng();
    let mut server_set: HashSet<String> = HashSet::with_capacity(servers.len());

    for srv in servers.iter() {
        server_set.insert(srv.to_owned());
        for _ in 1..rng.gen_range(150, 200) {
            ring.push(RingNode {
                i: rng.gen::<u32>(),
                srv: srv.to_owned(),
            });
        }
    }

    ring.sort();
    ring.dedup();

    let f = match hashfunc {
        HashFunction::OneAtATime => hash_one_at_a_time,
        HashFunction::FNV1a => hash_fnv_1a,
    };

    let c = Continuum {
        //server_set: server_set,
        ring: RwLock::new(ring),
        hashfunc: f,
    };

    Ok(c)
}

pub fn ketama_get_server(c: &Continuum, k: &str) -> String {
    let kh = (c.hashfunc)(k.as_bytes());
    return c.find_server(kh);
}

// ------------------------------------------------
// Hash Functions
// https://doc.rust-lang.org/book/second-edition/appendix-02-operators.html

// http://www.isthe.com/chongo/tech/comp/fnv/index.html
fn hash_fnv_1a(key: &[u8]) -> u32 {
    const PRIME: u32 = 16777619;
    const OFFSET: u32 = 2166136261;
    let mut hash: u32 = OFFSET;

    for b in key.iter() {
        hash ^= *b as u32;
        hash = hash.wrapping_mul(PRIME);
    }
    return hash;
}

// https://en.wikipedia.org/wiki/Jenkins_hash_function
fn hash_one_at_a_time(key: &[u8]) -> u32 {
    let mut hash: u32 = 0;

    for (i, _) in key.iter().enumerate() {
        hash = hash.wrapping_add(key[i] as u32);
        hash = hash.wrapping_add(hash.wrapping_shl(10));
        hash ^= hash.wrapping_shr(6);
    }

    hash = hash.wrapping_add(hash.wrapping_shl(3));
    hash ^= hash.wrapping_shr(11);
    hash.wrapping_add(hash.wrapping_shl(15));

    return hash;
}
