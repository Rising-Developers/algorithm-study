pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // 초기 벽의 높이는 Option 값이다.
        let mut latest_max_wall: Option<i32> = Option::None;

        let mut current_water = 0;
        let mut total_water = 0;

        // 순회를 한다.
        for (index, h) in height.iter().enumerate() {
            // 벽이 None 값이다?
            match latest_max_wall {
                None => {
                    // 벽 높이가 1이상이면?
                    // 현재 높이와 현재 높이 이후 오는 최대 높이 중 최솟값으로 갱신한다.
                    // 나머지 작업을 스킵한다.
                    if *h > 0 && index != height.len() - 1 {
                        latest_max_wall =
                            Some(Self::get_next_wall_height(*h, &height[index + 1..]));
                    }
                }
                Some(max_height) => {
                    // Some 값인 경우
                    // 현재 저장되어 있는 벽보다 높이가 높거나 같다?

                    // 현재 저장된 물의 량을 최종 물의 량에 더한다.
                    // 현재 저장된 물의 량을 초기화 한다.
                    // 현재 높이와 현재 높이 이후 오는 최대 높이 중 최솟값으로 갱신한다.

                    // 아니다
                    // 벽 높이를 새로 만난 벽으로 초기화 한다.

                    // 그게 아니다?
                    // 현재 물의 량에 현재 벽 높이 - 만난 벽 높이 의 값을 더한다.

                    if *h >= max_height {
                        total_water += current_water;
                        current_water = 0;

                        if index != height.len() - 1 {
                            latest_max_wall =
                                Some(Self::get_next_wall_height(*h, &height[index + 1..]));
                        }
                    } else {
                        current_water += max_height - h;
                    }
                }
            }
        }

        // 저장된 물을 전부 반환한다.

        total_water
    }

    fn get_next_wall_height(cur_height: i32, heights: &[i32]) -> i32 {
        let max_height = heights.iter().max().unwrap();

        cur_height.min(*max_height)
    }
}

#[test]
fn test_case_1() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

    assert_eq!(Solution::trap(height), 6);
}

#[test]
fn test_case_2() {
    let height = vec![4, 2, 0, 3, 2, 5];

    assert_eq!(Solution::trap(height), 9);
}
