use std::{fs::read_dir, path::Path};

#[test]
fn test() {
    let t = trybuild::TestCases::new();

    let ui_dir = Path::new("tests/ui");
    let dir = read_dir(ui_dir).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if name.ends_with(".rs") {
            let test_file = ui_dir.join(&name);
            if name.starts_with("ok") {
                t.pass(test_file);
            } else if name.starts_with("fail") {
                t.compile_fail(test_file);
            }
        }
    }
}
