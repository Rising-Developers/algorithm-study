/// # Gas station
/// n개의 주유소가 원형 경로를 따라 배치되어 있으며, 각 주유소에서 얻을 수 있는 가스의 양은 gas[i]입니다.
/// 당신은 무제한 연료 탱크를 가진 자동차를 가지고 있으며, i번째 주유소에서 (i + 1)번째 주유소로 이동하는 데 필요한 가스 비용은 cost[i]입니다. 당신은 주유소 중 하나에서 빈 탱크로 여행을 시작합니다.
/// 두 개의 정수 배열 gas와 cost가 주어졌을 때, 시계 방향으로 한 바퀴를 돌 수 있는 출발 주유소의 인덱스를 반환하세요. 만약 여행을 완료할 수 없다면 -1을 반환하세요. 해결책이 존재하는 경우 유일함이 보장됩니다.

struct Solution {}

// struct LeetCodeSolution {}
//
// impl LeetCodeSolution {
//     pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
//         match gas
//             .into_iter()
//             .zip(cost.into_iter()) // gas.iter 와 cost.iter 를 합쳐 튜플된 새로운 iter 를 생성한다.
//             .map(|(g, c)| g - c) // 튜플된 iter 를 순회하며 gas - cost를 한다.
//             .enumerate() // map 의 결과값을 (index, 값)으로 이루어진 튜플로 변환한다.
//             .fold((0, (0, 0)), |(s, pass @ (_, vm)), (i, v)| match s + v {
//                 // fold 는 자바스크립트의 reduce 와 유사하다.
//                 // 두 가지를 인자로 받는다. 초기값, 클로저를 받는다.
//                 // 클로저는 acc 와 Self::Item 을 인자로 받는다.
//                 // vm = gas[i] - cost[i]
//                 // 클로저
//                 s if s < vm => (s, (i as i32 + 1, s)),
//                 s => (s, pass),
//             }) {
//             (s, _) if s < 0 => -1,
//             (_, (im, _)) => im,
//         }
//     }
// }

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let start_positions = Self::find_start_position(&gas, &cost);

        if gas.len() == 1 && gas[0] - cost[0] >= 0 {
            return 0;
        }

        for start in start_positions {
            let mut index = start;

            // 초기 가스량을 계산
            let mut cur_gas = 0;

            let mut is_first_loop = true;

            loop {
                // index 가 start 와 같을 때 까지 순회
                if index == start && !is_first_loop {
                    return start as i32;
                }

                is_first_loop = false;

                cur_gas = Self::charge_gas(cur_gas, &gas, index);

                let required_cost = Self::get_cost(&cost, index);

                if cur_gas - required_cost >= 0 {
                    cur_gas = cur_gas - required_cost;

                    index = Self::get_next_index(index, gas.len());
                } else {
                    break;
                }
            }
        }

        -1
    }

    fn get_next_index(index: usize, len: usize) -> usize {
        let next_index = index + 1;
        return if next_index >= len { 0 } else { next_index };
    }
    fn get_cost(cost: &Vec<i32>, index: usize) -> i32 {
        *cost.get(index).unwrap()
    }

    fn charge_gas(cur_gas: i32, gas: &Vec<i32>, index: usize) -> i32 {
        cur_gas + gas.get(index).unwrap()
    }

    fn find_start_position(gas: &Vec<i32>, cost: &Vec<i32>) -> Vec<usize> {
        let mut start_position: Vec<(usize, i32)> = Vec::new();

        for (index, gas_amount) in gas.iter().enumerate() {
            let cost_amount = cost[index];

            if gas_amount - cost_amount > 0 {
                start_position.push((index, gas_amount - cost_amount));
            }
        }

        start_position.sort_by(|a, b| b.1.cmp(&a.1));

        let result: Vec<usize> = start_position.iter().map(|elem| elem.0).collect();

        result
    }
}

#[test]
fn test_case_1() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];

    assert_eq!(Solution::can_complete_circuit(gas, cost), 3);
}

#[test]
fn test_case_2() {
    let gas = vec![2, 3, 4];
    let cost = vec![3, 4, 3];

    assert_eq!(Solution::can_complete_circuit(gas, cost), -1);
}

#[test]
fn test_case_3() {
    let gas = vec![3, 1, 1];
    let cost = vec![1, 2, 2];

    assert_eq!(Solution::can_complete_circuit(gas, cost), 0);
}
