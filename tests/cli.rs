use std::process::Command;

#[test]
fn works() {
    assert_eq!(1, 1)
}

#[test]
fn runs() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}
