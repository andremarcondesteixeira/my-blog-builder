use assert_cmd::Command;
use assert_fs::{fixture::ChildPath, prelude::*, TempDir};
use predicates::prelude::*;
use std::fs;

#[test]
fn happy_path() -> Result<(), Box<dyn std::error::Error>> {
    let input = create_input_folder_and_files();
    let output_folder = create_output_folder();

    // Runs: `./my-blog-builder --input-folder src --output-folder out`
    let execution_result = execute_program(&input, output_folder);
    execution_result.success().stdout(predicate::eq("Done!"));

    /* Assert that input folder and files are left unchanged */
    let base_file_content = fs::read_to_string(input.template_file.path()).unwrap();
    assert_eq!(String::from("Hello #CONTENT"), base_file_content);

    let config_file_content = fs::read_to_string(input.config_file.path()).unwrap();
    assert_eq!(
        String::from("extends = \"./base.txt\""),
        config_file_content
    );

    let file_before_content = fs::read_to_string(input.file_before.path()).unwrap();
    assert_eq!(String::from("World!"), file_before_content);

    Ok(())
}

struct InputFolderAndFiles {
    input_folder: TempDir,
    template_file: ChildPath,
    config_file: ChildPath,
    file_before: ChildPath,
}

fn create_input_folder_and_files() -> InputFolderAndFiles {
    let input_folder = create_input_folder();
    let template_file = create_template_file(&input_folder);
    let config_file = create_config_file(&input_folder);
    let file_before = create_input_file_to_be_processed(&input_folder);
    InputFolderAndFiles {
        input_folder,
        template_file,
        config_file,
        file_before,
    }
}

fn create_input_folder() -> assert_fs::TempDir {
    let input_folder = assert_fs::TempDir::new().expect("Could not create input folder");
    input_folder
}

fn create_template_file(input_folder: &assert_fs::TempDir) -> assert_fs::fixture::ChildPath {
    let template_file = input_folder.child("base.txt");
    template_file
        .write_str("Hello #CONTENT")
        .expect("Could not create template file");
    template_file
}

fn create_config_file(input_folder: &assert_fs::TempDir) -> assert_fs::fixture::ChildPath {
    let config_file = input_folder.child("file.txt.parseme.toml");
    config_file
        .write_str("extends = \"./base.txt\"")
        .expect("Could not create config file");
    config_file
}

fn create_input_file_to_be_processed(
    input_folder: &assert_fs::TempDir,
) -> assert_fs::fixture::ChildPath {
    let file_before = input_folder.child("file.txt");
    file_before
        .write_str("World!")
        .expect("Could not create an input file to be processed");
    file_before
}

fn create_output_folder() -> TempDir {
    let output_folder = assert_fs::TempDir::new().unwrap();
    output_folder
}

fn execute_program(
    input: &InputFolderAndFiles,
    output_folder: TempDir,
) -> assert_cmd::assert::Assert {
    let mut cmd = Command::cargo_bin("my-blog-builder").unwrap();
    cmd.arg("--input-folder")
        .arg(input.input_folder.path())
        .arg("--output-folder")
        .arg(output_folder.path());
    let execution_result = cmd.assert();
    execution_result
}
