use std::collections::HashSet;

use rand::prelude::IteratorRandom;

pub struct RandomizedSet {
    set: HashSet<i32>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        RandomizedSet {
            set: HashSet::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        self.set.insert(val)
    }

    pub fn remove(&mut self, val: i32) -> bool {
        self.set.remove(&val)
    }

    pub fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        *self.set.iter().choose(&mut rng).unwrap()
    }
}

#[test]
fn test_case_1() {
    let mut obj = RandomizedSet::new();

    let res1 = obj.insert(10);

    let res2 = obj.insert(10);

    let res3 = obj.remove(10);

    let res4 = obj.insert(20);

    let res5 = obj.get_random();

    assert_eq!(res1, true);
    assert_eq!(res2, false);
    assert_eq!(res3, true);
    assert_eq!(res4, true);
    assert_eq!(res5, 20);
}

#[test]
fn test_case_2() {
    let mut obj = RandomizedSet::new();

    obj.insert(10);

    obj.insert(20);
    obj.insert(30);
    obj.insert(40);
    obj.insert(50);
    obj.insert(60);

    obj.insert(70);

    for _ in 1..50 {
        println!("{}", obj.get_random());
    }
}
