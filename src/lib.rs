use std::{
    env, error::Error, fs::File, io, os::unix::process::CommandExt, process::{Command, Stdio}
};

use camino::Utf8Path;

pub fn metabuild() {
    main().unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=post_build.rs");

    let out_dir = env::var("OUT_DIR")?;

    // Compile post_build.rs to binary
    let mut cmd = Command::new("rustc");
    cmd.args(&["--crate-name", "post_build_script_post_build"]);
    cmd.args(&["--edition", "2024"]);
    cmd.args(&["post_build.rs"]);
    cmd.args(&["--crate-type", "bin"]);
    cmd.args(&["--out-dir", &out_dir]);
    let status = cmd.status()?;
    if !status.success() {
        return Err(format!("{:?} failed: {:?}", cmd, status).into());
    }

    let post_build_script_path = Utf8Path::new(&out_dir).join("post_build_script_post_build");

    // Watch current build process until it finishes
    let mut cmd = Command::new("node");
    cmd.args(&["--input-type=module", "--eval"]);
    cmd.arg(r#"
        const args = process.argv.slice(1);
        console.log("PID is %s", process.pid);
        console.log("Parent PID is %s", process.ppid);
        const postBuildScriptPath = args[0];
        console.log("Post-build script path is %s", postBuildScriptPath);
    "#);
    cmd.args(&[&post_build_script_path]);
    let stdout = File::create("/workspaces/post-build/node-stdout.log")?;
    let stderr = File::create("/workspaces/post-build/node-stderr.log")?;
    cmd.stdin(Stdio::null());
    cmd.stdout(stdout);
    cmd.stderr(stderr);
    let pre_exec_hook = || -> io::Result<()> {
        unsafe { libc::setsid() };
        Ok(())
    };
    unsafe { cmd.pre_exec(pre_exec_hook) };
    cmd.spawn()?;

    Ok(())
}
