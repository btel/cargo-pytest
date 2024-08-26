fn main() {
    let o = std::process::Command::new("pytest").arg("tests").output().unwrap();
    let stdout = String::from_utf8(o.stdout).unwrap();
    let stderr = String::from_utf8(o.stderr).unwrap();
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    assert!(o.status.success())
}
