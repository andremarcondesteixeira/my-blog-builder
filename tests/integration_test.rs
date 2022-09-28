use assert_cmd::Command;
use assert_fs::{fixture::ChildPath, prelude::*, TempDir};
use predicates::prelude::*;
use std::fs;

#[test]
fn happy_path() {
    let input = create_input_folder_and_files();

    // Runs: `./my-blog-builder --input-folder {input folder path} --output-folder {output folder path}`
    let execution_result = execute_program(&input);
    execution_result.success().stdout(predicate::eq("Done!"));

    input_folder_and_files_should_be_left_unchanged(&input);
    output_folder_and_files_should_be_generated_correctly(&input);
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

fn execute_program(input: &InputFolderAndFiles) -> assert_cmd::assert::Assert {
    let mut cmd = Command::cargo_bin("my-blog-builder").unwrap();
    cmd.arg("--input-folder")
        .arg(input.input_folder.path())
        .arg("--output-folder")
        .arg("out");
    let execution_result = cmd.assert();
    execution_result
}

fn input_folder_and_files_should_be_left_unchanged(input: &InputFolderAndFiles) {
    base_file_should_be_unchanged(input);
    config_file_should_be_unchanged(input);
    original_file_should_be_unchanged(input);
    no_new_files_should_have_been_created_in_the_input_folder(input);
}

fn base_file_should_be_unchanged(input: &InputFolderAndFiles) {
    let base_file_content =
        fs::read_to_string(input.template_file.path()).expect("base file not found");
    assert_eq!(String::from("Hello #CONTENT"), base_file_content);
}

fn config_file_should_be_unchanged(input: &InputFolderAndFiles) {
    let config_file_content =
        fs::read_to_string(input.config_file.path()).expect("config file not found");
    assert_eq!(
        String::from("extends = \"./base.txt\""),
        config_file_content
    );
}

fn original_file_should_be_unchanged(input: &InputFolderAndFiles) {
    let file_before_content =
        fs::read_to_string(input.file_before.path()).expect("original file not found");
    assert_eq!(String::from("World!"), file_before_content);
}

fn no_new_files_should_have_been_created_in_the_input_folder(input: &InputFolderAndFiles) {
    let dir_content = fs::read_dir(input.input_folder.path()).unwrap();
    let amount_of_childs = dir_content.count();
    assert_eq!(3, amount_of_childs);
}

fn output_folder_and_files_should_be_generated_correctly(input: &InputFolderAndFiles) {
    let output_folder = input.input_folder.parent().unwrap().join("out");
    let output_folder_path = output_folder.as_path();

    assert_eq!(true, output_folder_path.is_dir());
    assert_eq!(1, fs::read_dir(output_folder_path).unwrap().count());

    let folder_contents = fs::read_dir(output_folder_path).unwrap();
    let created_file = folder_contents.enumerate().next().unwrap();
    let created_file_path = (created_file.1).unwrap().path();
    let created_file_content = fs::read_to_string(created_file_path).unwrap();

    assert_eq!("Hello World!", created_file_content);
}
