use std::{fs::read_dir, path::Path};

#[test]
fn expand_examples() {
    let current_path = snapbox::current_dir!();
    println!("current_path: {}", current_path.display());
    let example_dir = current_path.join("../../heapuse/examples");
    let dir = read_dir(example_dir).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let mut name = entry.file_name().into_string().unwrap();
        if name.ends_with(".rs") {
            name.truncate(name.len() - 3);
            let expected = snapbox::Data::read_from(
                Path::new(&format!(
                    "{}/expand/{}.stdout",
                    current_path.display(),
                    &name
                )),
                None,
            );
            // cargo expand path::to::module
            snapbox::cmd::Command::new("cargo")
                .args(["expand", "-p", "heapuse", "--example", &name])
                .assert()
                .success()
                .stdout_matches(expected);
        }
    }
}
