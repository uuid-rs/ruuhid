use rustc_version;
use std::{env, error, fs, io, path};

fn main() -> Result<(), Box<error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = path::Path::new(&out_dir).join("build_context_compute.rs");
    let mut f = fs::File::create(&dest_path)?;
    let rustc_version = rustc_version::version()?;
    let version_context: u16 = (rustc_version.major * 1_000_000
        + rustc_version.minor * 1_000
        + rustc_version.patch) as u16;

    let mut buffer = String::new();
    buffer.push_str(
        r#"
        const fn context_value() -> u16 {
    "#,
    );

    buffer.push_str(&version_context.to_string());
    buffer.push_str(
        r#"
    }"#,
    );

    io::Write::write_all(&mut f, &buffer.as_bytes())?;

    Ok(())
}
