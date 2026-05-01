use std::path::PathBuf;
use std::process::Command;

pub fn configure_test_command(command: &mut Command) {
    if std::env::var_os("LLVM_PROFILE_FILE").is_some() {
        let profile_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("llvm-cov-target")
            .join("quaid-subprocess-%p-%m.profraw");
        if let Some(parent) = profile_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        command.env("LLVM_PROFILE_FILE", profile_path);
    }
}
