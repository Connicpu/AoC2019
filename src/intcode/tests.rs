use super::parse;

#[cfg(test)]
#[test]
fn test_parse() {
    assert_eq!(parse("1"), vec![1]);
    assert_eq!(parse("-1"), vec![-1]);
    assert_eq!(parse("-1,"), vec![-1]);
    assert_eq!(parse("-1\n2,3,4"), vec![-1, 2, 3, 4]);
}
