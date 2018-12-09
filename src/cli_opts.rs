#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "uuid")]
pub struct CliOpts {
    /// Decode a `uuid`.
    #[structopt(name = "decode", short = "-d")]
    decode: bool,
    #[structopt(short = "-v")]
    uuid_version: String,
}