extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::path::PathBuf;


#[derive(Debug)]
struct Repository {
    path: String,
    passphrase: String,
}

fn read_repo_config() -> Repository{
    let passphrase: String = env::var("BORG_PASSPHRASE").unwrap();
    let repo_path = env::var("BORG_REPOSITORY").unwrap();
    Repository { path: repo_path, passphrase: passphrase }
}

#[derive(Debug)]
struct Archive {
    prefix: String,
    base_paths: Vec<PathBuf>,
    exclude: Vec<String>,
}
// TODO
// Create borg struct that implements functions to interact with archives
// Keep path to binary as Field

fn create_archive(repo: &Repository, archive: &Archive) {
    use std::process::{Command, Stdio};

    let full_archive_path = format!("{}::{}-{{now:%Y-%m-%dT%H:%M}}",
                                    repo.path, archive.prefix);

    let borg_executable: PathBuf = PathBuf::from("/usr/bin/borg");
    if !(borg_executable.exists() && borg_executable.is_file()) {
        println!("Borg executable not found");
        return
    }
    let child = Command::new(borg_executable.as_os_str())
                            .arg("create")
                            .arg("--info")
                            .arg("--stats")
                            .arg(full_archive_path)
                            .args(archive.base_paths.into_iter().map(|x| x.to_os_str()))
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .spawn()
                            .expect("failed to execute child");
    let output = child.wait_with_output().expect("failed to wait on child");

    if output.status.success() {
        // return
    }
    println!("Errorcode {}", output.status);
    if !output.stdout.is_empty() {
        let stdout = std::str::from_utf8(&output.stdout).unwrap();
        println!("Stdout:\n{}", stdout);
    }
    if !output.stderr.is_empty() {
        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        println!("Stderr:\n{}", stderr);
    }
}

fn purge_archives(repo: &Repository, prefix: &String) {

}

fn main() {
    dotenv().ok();
    let repo = read_repo_config();
    println!("Config: {:?}", &repo);
    let archive = Archive { prefix: "test".to_string(),
                            base_paths: vec!(PathBuf::from("/home/don/tmp/taskd")),
                            exclude: vec![] };
    create_archive(&repo, &archive);
}
