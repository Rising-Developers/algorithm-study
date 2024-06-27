use std::collections::HashMap;

struct TwoSum {
    numbers: Vec<i32>,
    sum_map: HashMap<i32, bool>,
}

impl TwoSum {
    fn new() -> Self {
        TwoSum {
            numbers: Vec::new(),
            sum_map: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        self.numbers.push(number);

        if self.numbers.len() == 1 {
            self.sum_map.insert(number, true);
        } else {
            let iter = self.sum_map.clone();

            for (key, value) in iter {
                self.sum_map.insert(key + number, true);
            }
        }
    }

    fn find(&self, value: i32) -> bool {
        if self.numbers.len() <= 1 {
            return false;
        }
        *self.sum_map.get(&value).unwrap_or_else(|| &false)
    }
}
