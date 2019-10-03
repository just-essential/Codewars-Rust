pub fn get_count(string: &str) -> usize {
    string
        .chars()
        .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c))
        .count()
}

#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}
