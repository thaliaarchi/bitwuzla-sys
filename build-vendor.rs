use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use copy_dir::copy_dir;

pub struct BitwuzlaBuild {
    src_dir: PathBuf,
    out_dir: PathBuf,
}

impl BitwuzlaBuild {
    pub fn new() -> Self {
        Self {
            src_dir: Path::new(env!("CARGO_MANIFEST_DIR")).join("bitwuzla"),
            out_dir: Path::new(&env::var_os("OUT_DIR").expect("`OUT_DIR` not set"))
                .join("vendor-build"),
        }
    }

    pub fn prerequisites(self) -> Self {
        if !self.out_dir.exists() {
            copy_dir(&self.src_dir, &self.out_dir)
                .expect("failed to copy Bitwuzla sources to `OUT_DIR`");
        }

        self
    }

    pub fn build(self) -> Self {
        self.run_command(
            "Configure Bitwuzla",
            Command::new("python3")
                .arg(self.out_dir.join("configure.py"))
                .current_dir(&self.out_dir),
        );

        self.run_command(
            "Build Bitwuzla",
            Command::new("meson")
                .arg("install")
                .args(["--destdir", "install"])
                .current_dir(self.out_dir.join("build")),
        );

        println!(
            "cargo:rustc-link-search=native={}",
            self.out_dir.join("build/install/usr/local/lib").display(),
        );
        println!("cargo:rustc-link-lib=static=bitwuzla");
        println!("cargo:rustc-link-lib=gmp");

        self
    }

    fn run_command(&self, description: &str, command: &mut Command) {
        println!("*** {}", description);

        let status = command.status().unwrap();

        if !status.success() {
            panic!(
                "*** ERROR in action `{}`, exit status {}\n*** Command: {:?}",
                description, status, command,
            );
        }
    }
}
