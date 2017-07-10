
/// Print stderr/stdout of process if not a success
pub fn print_if_failure(output: ::std::process::Output) {
    if output.status.success() {
        return
    }
    println!("Errorcode {}", output.status);
    if !output.stdout.is_empty() {
        let stdout = ::std::str::from_utf8(&output.stdout).unwrap();
        println!("Stdout:\n{}", stdout);
    }
    if !output.stderr.is_empty() {
        let stderr = ::std::str::from_utf8(&output.stderr).unwrap();
        println!("Stderr:\n{}", stderr);
    }
}

