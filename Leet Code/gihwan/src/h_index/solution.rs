pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut citations_clone = citations.clone();
    let n = citations.len();
    citations_clone.sort_unstable();
    for (i, &citation) in citations_clone.iter().enumerate() {
        if citation as usize >= n - i {
            return (n - i) as i32;
        }
    }
    0
}

#[test]
fn test_case_1() {
    let citations = vec![3, 0, 6, 1, 5];

    assert_eq!(h_index(citations), 3);
}

#[test]
fn test_case_2() {
    let citations = vec![1, 3, 1];

    assert_eq!(h_index(citations), 1);
}

#[test]
fn test_case_3() {
    let citations = vec![100];

    assert_eq!(h_index(citations), 100);
}
