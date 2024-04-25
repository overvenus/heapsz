use snapbox::file;

#[test]
fn expand() {
    snapbox::cmd::Command::new("cargo")
        .args(&["expand", "-p", "heapuse-example"])
        .assert()
        .success()
        .stdout_matches(file!["expand/expand.stdout"]);
}
