struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut zigzags: Vec<_> = (0..num_rows)
            // 두 이터레이터를 합쳐 새로운 이터레이터를 생성하는 메서드
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .collect();
        zigzags.sort_by_key(|&(row, _)| row);
        zigzags.into_iter().map(|(_, c)| c).collect()
    }
}
