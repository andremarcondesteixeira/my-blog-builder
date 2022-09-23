use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn happy_path() -> Result<(), Box<dyn std::error::Error>> {
    let input_folder = assert_fs::TempDir::new().unwrap();
    let output_folder = assert_fs::TempDir::new().unwrap();

    let base_file = input_folder.child("base.txt");
    base_file.write_str("Hello #CONTENT")?;

    let file = input_folder.child("file.txt");
    file.write_str("World!")?;

    // my-blog-builder --input-folder src --output-folder out
    let mut cmd = Command::cargo_bin("my-blog-builder")?;
    cmd.arg("--input-folder")
        .arg(input_folder.path())
        .arg("--output-folder")
        .arg(output_folder.path());

    cmd.assert().success().stdout(predicate::eq("Done!"));

    Ok(())
}
