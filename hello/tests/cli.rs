use std::process::Command;
use assert_cmd::cargo::CommandCargoExt;
use pretty_assertions::assert_eq; 

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("fail");
    
    assert!(output.status.success());
    let std_out = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(std_out, "Hello, world!\n");
    
}