const OVER: i32 = 1000;

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    filter(nums, val);
    nums.len() as i32
}

fn filter(nums: &mut Vec<i32>, val: i32) {
    let mut iter = nums.clone().into_iter();

    nums.clear();

    loop {
        let next = iter.next().unwrap_or_else(|| OVER);

        if next == OVER {
            return;
        }

        if val != next {
            nums.push(next);
        }
    }
}
