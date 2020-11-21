use std::io::prelude::Read;
use std::path::PathBuf;
use std::process::Command;

pub struct ExecJar;

impl ExecJar {
  pub fn new() -> ExecJar {
    ExecJar {}
  }

  pub fn execute(&self, args: &[String]) {
    let mut path = self.get_cur_exec_path();
    path.push("javaw");

    let mut cmd_line_args = Vec::<String>::new();
    cmd_line_args.push("-jar".to_string());
    cmd_line_args.extend_from_slice(args);

    let mut java_jar_command = Command::new(path);
    java_jar_command.args(&cmd_line_args);
    let output = java_jar_command
      .spawn()
      .expect("Failed to execute java -jar");

    let mut string_buffer = String::new();
    output
      .stdout
      .unwrap()
      .read_to_string(&mut string_buffer)
      .unwrap_or(0);
    println!("{}", string_buffer);
  }

  fn get_cur_exec_path(&self) -> PathBuf {
    let current_dir = std::env::current_exe().unwrap_or(PathBuf::from(""));
    PathBuf::from(current_dir.parent().unwrap())
  }
}
