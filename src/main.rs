extern crate dotenv;

use dotenv::dotenv;
use std::path::PathBuf;

mod borg;
mod term;


fn main() {
    dotenv().ok();
    let repo = borg::read_repo_config();
    if !repo.is_valid() {
        println!("Borg executable not found");
        return;
    }
    let archive = borg::Archive {
        prefix: "test".to_string(),
        base_paths: vec!(PathBuf::from("/home/don/tmp/taskd")),
        exclude: vec![]
    };
    repo.create_archive(&archive);
    repo.purge_archives(&archive.prefix);
}
