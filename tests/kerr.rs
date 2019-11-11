use kerr::KErr;

#[test]
fn aaa_kerr() {
    let mut e = KErr::new("ABC");
    assert_eq!(format!("{}", e), "ABC");
    assert_eq!(format!("{:?}", e), r#"KErr{ err:"ABC", chain:[] }"#);
    e = e.pre("a");
    assert_eq!(format!("{}", e), "a : ABC");
    e = e.pre("b");
    assert_eq!(format!("{}", e), "b : a : ABC");
    e = e.pre("c");
    assert_eq!(format!("{}", e), "c : b : a : ABC");
    assert_eq!(format!("{:?}", e), r#"KErr{ err:"ABC", chain:[ c, b, a ] }"#);

    match e.err.as_str() {
        "ABC" => {}
        _ => { panic!("inconceivable"); }
    }

    assert_eq!(format!("{}", e), "c : b : a : ABC");
}

