/// Roman to integer
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000

struct Solution {}

struct LeetCodeSolution {}

impl LeetCodeSolution {
    /// as_bytes 사용 이유:
    ///저는 보통 LeetCode에 솔루션을 게시할 때 // 입력은 ASCII => 문자는 바이트입니다와 같은 주석을 추가합니다.
    /// 하지만 이번에는 그렇지 않으므로 시간을 내서 설명하겠습니다.
    /// chars()는 기본 배열을 UTF-8로 구문 분석하므로 반복할 때 각 문자를 분석하여 (UTF-8은 가변 길이 인코딩이므로)
    /// 가져올 바이트 수를 결정하고 4바이트 UTF-32 값(Rust의 문자 형식)으로 변환해야 합니다.
    /// 하지만 입력이 모두 1바이트인 ASCII 문자로만 구성되어 있다는 것을 알기 때문에 기본 바이트 배열을 직접 반복하고 구문 분석을 우회할 수 있습니다.
    /// 사소한 최적화일 수도 있지만, 이렇게 간단한데 왜 안 될까요?
    pub fn roman_to_int(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .fold([0; 4], |[m, c, x, i], b| match *b {
                b'M' => [m + 1000 - c, 0, x, i],
                b'D' => [m + 500 - c, 0, x, i],
                b'C' => [m, c + 100 - x, 0, i],
                b'L' => [m, c + 50 - x, 0, i],
                b'X' => [m, c, x + 10 - i, 0],
                b'V' => [m, c, x + 5 - i, 0],
                _ => [m, c, x, i + 1],
            })
            .into_iter()
            .sum::<i32>()
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let (mut m, mut c, mut x, mut i) = (0, 0, 0, 0);

        for b in s.chars() {
            match b {
                'M' => {
                    m += 1000 - c;
                    c = 0;
                }
                'D' => {
                    m += 500 - c;
                    c = 0;
                }
                'C' => {
                    c += 100 - x;
                    x = 0;
                }
                'L' => {
                    c += 50 - x;
                    x = 0;
                }
                'X' => {
                    x += 10 - i;
                    i = 0;
                }
                'V' => {
                    x += 5 - i;
                    i = 0;
                }
                _ => {
                    i += 1;
                }
            }
        }
        m + c + x + i
    }
}

#[test]
fn test_case_1() {
    let s = String::from("III");

    assert_eq!(Solution::roman_to_int(s), 3);
}

#[test]
fn test_case_2() {
    let s = String::from("LVIII");

    assert_eq!(Solution::roman_to_int(s), 58);
}
#[test]
fn test_case_3() {
    let s = String::from("MCMXCIV");

    assert_eq!(Solution::roman_to_int(s), 1994);
}
