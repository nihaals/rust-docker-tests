use std::process::Command;

enum BuildImage {
    Bullseye,
    Alpine,
}

impl BuildImage {
    fn docker_image(&self) -> &str {
        match self {
            BuildImage::Bullseye => "rust:1-bullseye",
            BuildImage::Alpine => "rust:1-alpine",
        }
    }
}

enum RunImage {
    RustBullseye,
    RustAlpine,
    Bullseye,
    Alpine,
    BusyBox,
    Scratch,
}

impl RunImage {
    fn docker_image(&self) -> &str {
        match self {
            RunImage::RustBullseye => "rust:1-bullseye",
            RunImage::RustAlpine => "rust:1-alpine",
            RunImage::Bullseye => "debian:bullseye",
            RunImage::Alpine => "alpine",
            RunImage::BusyBox => "busybox",
            RunImage::Scratch => "scratch",
        }
    }
}

enum BuildError {
    Unknown,
}

enum RunError {
    NotFound,
    SegFault,
    Unknown,
}

struct TestCase {
    /// Contains no `/`
    dir_name: String,
    static_flags: bool,
    build_image: BuildImage,
    run_image: RunImage,
}

impl TestCase {
    fn image_tag(&self) -> String {
        let build_image = match self.build_image {
            BuildImage::Bullseye => "debian-bullseye",
            BuildImage::Alpine => "alpine",
        };
        let run_image = match self.run_image {
            RunImage::RustBullseye => "rust-debian-bullseye",
            RunImage::RustAlpine => "rust-alpine",
            RunImage::Bullseye => "debian-bullseye",
            RunImage::Alpine => "alpine",
            RunImage::BusyBox => "busybox",
            RunImage::Scratch => "scratch",
        };
        format!(
            "{}-{}-{}-{}",
            self.dir_name, build_image, run_image, self.static_flags
        )
    }

    fn build_image(&self) -> Result<(), BuildError> {
        let rust_flags = if self.static_flags {
            "-Ctarget-feature=+crt-static"
        } else {
            ""
        };
        Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(self.image_tag())
            .arg("--build-arg")
            .arg(format!("BASE_IMAGE={}", self.build_image.docker_image()))
            .arg("--build-arg")
            .arg(format!("RUN_IMAGE={}", self.run_image.docker_image()))
            .arg("--build-arg")
            .arg(format!("TEST_CASE={}", self.dir_name))
            .arg("--build-arg")
            .arg(format!("RUST_FLAGS={}", rust_flags))
            .arg(".")
            .current_dir("../test-cases")
            .spawn()
            .map_err(|_| BuildError::Unknown)?
            .wait()
            .map_err(|_| BuildError::Unknown)?
            .success()
            .then(|| ())
            .ok_or(BuildError::Unknown)
    }

    fn run_image(&self) -> Result<(), RunError> {
        Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg(self.image_tag())
            .spawn()
            .map_err(|_| RunError::Unknown)?
            .wait()
            .map_err(|_| RunError::Unknown)?
            .success()
            .then(|| ())
            .ok_or(RunError::Unknown)
    }
}

fn main() {}
