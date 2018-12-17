#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "uuid")]
pub struct CliOpts {
    #[structopt(
        long = "--hex",
        short = "-x"
    )]
    hex: bool,
    #[structopt(
        long = "--md5",
        short = "-m"
    )]
    md5: bool,
    #[structopt(
        long = "--name",
        short = "-N"
    )]
    name: Option<String>,
    #[structopt(
        long = "--namespace",
        short = "-n"
    )]
    namespace: Option<uuid::Uuid>,
    #[structopt(
        long = "--random",
        short = "-r"
    )]
    random: bool,
    #[structopt(
        long = "--sha1",
        short = "-s"
    )]
    sha1: bool,
    #[structopt(
        long = "--time",
        raw(
            conflicts_with_all = "&[\"random\"]"
        ),
        short = "-t"
    )]
    time: bool,
}