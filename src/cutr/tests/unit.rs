use cutr::parse_pos;

#[test]
fn test_parse_pos() {
    // The empty string is an error
    assert!(parse_pos("").is_err());

    // Zero is an error
    let res = parse_pos("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

    let res = parse_pos("0-1");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

    // A leading "+" is an error
    let res = parse_pos("+1");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1\"",);

    let res = parse_pos("+1-2");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1-2\"",);

    let res = parse_pos("1-+2");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"1-+2\"",);

    // Any non-number is an error
    let res = parse_pos("a");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

    let res = parse_pos("1,a");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

    let res = parse_pos("1-a");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"1-a\"",);

    let res = parse_pos("a-1");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a-1\"",);

    // Wonky ranges
    let res = parse_pos("-");
    assert!(res.is_err());

    let res = parse_pos(",");
    assert!(res.is_err());

    let res = parse_pos("1,");
    assert!(res.is_err());

    let res = parse_pos("1-");
    assert!(res.is_err());

    let res = parse_pos("1-1-1");
    assert!(res.is_err());

    let res = parse_pos("1-1-a");
    assert!(res.is_err());

    // First number must be less than second
    let res = parse_pos("1-1");
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "First number in range (1) must be lower than second number (1)"
    );

    let res = parse_pos("2-1");
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "First number in range (2) must be lower than second number (1)"
    );

    // All the following are acceptable
    let res = parse_pos("1");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vec![0..1]);

    let res = parse_pos("01");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vec![0..1]);

    let res = parse_pos("1,3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vec![0..1, 2..3]);

    let res = parse_pos("001,0003");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vec![0..1, 2..3]);

    let res = parse_pos("1-3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vec![0..3]);

    let res = parse_pos("0001-03");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vec![0..3]);

    let res = parse_pos("1,7,3-5");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

    let res = parse_pos("15,19-20");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vec![14..15, 18..20]);
}
