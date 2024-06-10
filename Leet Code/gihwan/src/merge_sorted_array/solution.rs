const OVER: i32 = 1000;

pub fn merge_with_merge_sort(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut nums1_length: i32 = m;
    let mut nums2_length: i32 = n;

    let mut nums1_iter = nums1.clone().into_iter();
    let mut nums2_iter = nums2.clone().into_iter();

    let mut nums1_next = nums1_iter.next().unwrap_or_else(|| OVER);
    let mut nums2_next = nums2_iter.next().unwrap_or_else(|| OVER);

    nums1_length -= 1;
    nums2_length -= 1;

    nums1.clear();

    loop {
        if nums1_length == -1 || nums2_length == -1 {
            if nums1_length == -1 {
                loop {
                    if nums2_length == -1 {
                        break;
                    }
                    nums1.push(nums2_next);
                    nums2_next = nums2_iter.next().unwrap_or_else(|| OVER);
                    nums2_length -= 1;
                }
            } else {
                loop {
                    if nums1_length == -1 {
                        break;
                    }
                    nums1.push(nums1_next);
                    nums1_next = nums1_iter.next().unwrap_or_else(|| OVER);
                    nums1_length -= 1;
                }
            }
            break;
        }

        if nums1_next < nums2_next {
            nums1.push(nums1_next);
            nums1_next = nums1_iter.next().unwrap_or_else(|| OVER);
            nums1_length -= 1;
        } else {
            nums1.push(nums2_next);
            nums2_next = nums2_iter.next().unwrap_or_else(|| OVER);
            nums2_length -= 1;
        }
    }

    println!("{:?}", nums1);
}

pub fn merge_with_standard_sort(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let nums1_clone = nums1.clone();
    let nums2_clone = nums2.clone();

    let mut nums1_iter = nums1_clone.into_iter();
    let mut nums2_iter = nums2_clone.into_iter();

    let mut nums1_length = m;
    let mut nums2_length = n;

    nums1.clear();

    loop {
        if nums1_length == 0 && nums2_length == 0 {
            break;
        }
        if nums1_length != 0 {
            let element = nums1_iter.next().expect("");
            nums1.push(element);
            nums1_length -= 1;
        }

        if nums2_length != 0 {
            let element = nums2_iter.next().expect("");
            nums1.push(element);
            nums2_length -= 1;
        }
    }

    nums1.sort();
}
