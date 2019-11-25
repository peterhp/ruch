use std::collections::HashMap;

use crate::cache::types::{RString, RList, RHash};

const INITIAL_HASH_SLOTS: usize = 32;
const DEFAULT_PARTITION_SIZE: usize = 16;

type RKey = RString;

enum RValue {
    RString(RString),
    RList(RList),
    RHash(RHash),
}

struct RDictEntry {
    key: RKey,
    val: RValue,
}

pub struct RDict {
    tbl: HashMap<RKey, RDictEntry>,
}

pub struct RDb {
    dicts: Vec<RDict>,
}

impl RValue {
    #[inline]
    #[allow(dead_code)]
    fn new_string(val: RString) -> Self {
        RValue::RString(val)
    }

    #[inline]
    #[allow(dead_code)]
    fn new_list(val: RList) -> Self {
        RValue::RList(val)
    }

    #[inline]
    #[allow(dead_code)]
    fn new_hash(val: RHash) -> Self {
        RValue::RHash(val)
    }
}

impl RDict {
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Self {
        RDict {
            tbl: HashMap::with_capacity(INITIAL_HASH_SLOTS),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_string(&mut self, key: RString, val: RString) -> bool {
        if !self.tbl.contains_key(&key) {
            let entry = RDictEntry {
                key,
                val: RValue::RString(val),
            };
            self.tbl.insert(entry.key.clone(), entry);

            return true;
        }
        false
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_string(&self, key: /*&RString*/&str) -> Option<RString> {
        if let Some(entry) = self.tbl.get(key) {
            if let RValue::RString(val) = &entry.val {
                return Some(val.clone())
            }
        }
        None
    }
}

impl RDb {
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut dicts = Vec::with_capacity(DEFAULT_PARTITION_SIZE);
        for _ in 0..DEFAULT_PARTITION_SIZE {
            dicts.push(RDict::new())
        }

        RDb {
            dicts,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn select(&mut self, pid: usize) -> Option<&mut RDict> {
        self.dicts.get_mut(pid)
    }
}
