use pretty_assertions::assert_eq; 
use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("fail");
    
    assert!(output.status.success());
    let std_out = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(std_out, "Hello, world!\n");    
}


#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();

    cmd.assert().success();
}

#[test]
fn false_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();

    cmd.assert().failure();
}
