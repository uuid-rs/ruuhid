use std::path;

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "uuid")]
pub struct CliOpts {
    #[structopt(short = "-n")]
    count: usize,
    /// Decode a `uuid`.
    #[structopt(name = "decode", short = "-d")]
    decode: bool,
    #[structopt(short = "-o", parse(from_os_str))]
    filename: path::PathBuf,
    #[structopt(short = "-F")]
    format: String,
    #[structopt(short = "-m")]
    multicast: bool,
    #[structopt(name = "namespace name")]
    namespace: String,
    #[structopt(short = "-1")]
    reset_context: bool,
    /// Specify a uuid version.
    #[structopt(short = "-v", name = "uuid version")]
    uuid_version: Option<String>,
}