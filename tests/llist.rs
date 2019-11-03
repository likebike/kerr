use kerr::llist::LList;

#[test]
fn aaa_llist() {
    let l : LList<String> = LList::new();
    assert_eq!(format!("{}", l), "[]");
    assert_eq!(format!("{:?}", l), "LList[]");
    let l = l.prepend("a".to_string());
    assert_eq!(format!("{}", l), "[ a ]");
    assert_eq!(format!("{:?}", l), r#"LList[ "a" ]"#);
    let l = l.prepend("b".to_string());
    assert_eq!(format!("{}", l), "[ b a ]");
    assert_eq!(format!("{:?}", l), r##"LList[ "b" "a" ]"##);

    let mut l : LList<i32> = LList::new();
    for i in 5..25 { l = l.prepend(i) }
    assert_eq!(format!("{}", l), "[ 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 ]");
}

