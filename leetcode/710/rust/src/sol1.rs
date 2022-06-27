pub struct Solution {
    n: i32,
    backlist: Vec<i32>,
    rand: rand::rngs::ThreadRng,
    bklen: i32,
    map: HashMap<i32, i32>,
}
use std::collections::{HashMap, HashSet};

use rand::{self, Rng};
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut set: HashSet<i32> = HashSet::new();
        let mut ind = n - blacklist.len() as i32;
        let thr = ind.clone();
        for i in blacklist.iter() {
            if i >= &ind {
                set.insert(i.clone());
            }
        }
        for i in blacklist.iter() {
            if i < &thr {
                while set.contains(&ind) {
                    ind += 1;
                }
                map.insert(i.clone(), ind);
                ind += 1;
            }
        }

        Solution {
            n,
            map,
            backlist: blacklist.clone(),
            rand: rand::thread_rng(),
            bklen: blacklist.len() as i32,
        }
    }

    fn pick(&mut self) -> i32 {
        let ri = self.rand.gen_range(0, (self.n - self.bklen));
        if let Some(mapi) = self.map.get(&ri) {
            return mapi.clone();
        }
        return ri;
    }
}
