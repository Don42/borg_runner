use std::env;
use std::path::PathBuf;

use ::term::print_if_failure;

/// This represents configuration to connect to a borg repository
#[derive(Debug)]
pub struct Repository {
    borg_executable: PathBuf,
    path: String,
    passphrase: String,
}

/// Read repository from configuration
pub fn read_repo_config() -> Repository {
    let passphrase: String = env::var("BORG_PASSPHRASE").unwrap();
    let repo_path = env::var("BORG_REPOSITORY").unwrap();
    Repository {
        path: repo_path,
        passphrase: passphrase,
        borg_executable: PathBuf::from("/usr/bin/borg"),
    }
}

#[derive(Debug)]
pub struct Archive {
    pub prefix: String,
    pub base_paths: Vec<PathBuf>,
    pub exclude: Vec<String>,
}

impl Repository {
    pub fn is_valid(&self) -> bool {
        self.borg_executable.exists() && self.borg_executable.is_file()
    }
    pub fn create_archive(&self, archive: &Archive) {
        use std::process::{Command, Stdio};

        let full_archive_path = format!("{}::{}-{{now:%Y-%m-%dT%H:%M}}",
                                        self.path, archive.prefix);

        let child = Command::new(self.borg_executable.as_os_str())
            .arg("create")
            .arg("--info")
            .arg("--stats")
            .arg(full_archive_path)
            .args(archive.base_paths.clone().into_iter().map(|x| x.into_os_string()))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute create");
        let output = child.wait_with_output().expect("failed to wait on create");
        print_if_failure(output);
    }

    /// Purge old archives from repository
    pub fn purge_archives(&self, prefix: &String) {
        use std::process::{Command, Stdio};

        let child = Command::new(self.borg_executable.as_os_str())
            .arg("prune")
            .arg("-v")
            .arg("--list")
            .arg(self.path.clone())
            .arg("--prefix")
            .arg(prefix)
            .arg("--keep-daily=7")
            .arg("--keep-weekly=4")
            .arg("--keep-monthly=6")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute purge");
        let output = child.wait_with_output().expect("failed to wait on purge");
        print_if_failure(output);
    }

}
