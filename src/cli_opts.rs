#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "uuid")]
pub struct CliOpts {
    #[structopt(long = "--hex", short = "-x")]
    hex: bool,
    #[structopt(
        long = "--md5",
        raw(
            conflicts_with_all = "&[\
                                  \"random\",\
                                  \"sha1\",\
                                  \"time\"\
                                  ]",
            requires_all = "&[\
                                    \"name\",
                                    \"namespace\"]"
        ),
        short = "-m"
    )]
    md5: bool,
    #[structopt(long = "--name", short = "-N")]
    name: Option<String>,
    #[structopt(long = "--namespace", short = "-n")]
    namespace: Option<uuid::Uuid>,
    #[structopt(
        long = "--random",
        raw(conflicts_with_all = "&[\
                                  \"md5\",\
                                  \"sha1\",\
                                  \"time\"\
                                  ]"),
        short = "-r"
    )]
    random: bool,
    #[structopt(
        long = "--sha1",
        raw(
            conflicts_with_all = "&[\
                                  \"md5\",\
                                  \"random\",\
                                  \"time\"\
                                  ]",
            requires_all = "&[\
                                    \"name\",
                                    \"namespace\"]"
        ),
        short = "-s"
    )]
    sha1: bool,
    #[structopt(
        long = "--time",
        raw(conflicts_with_all = "&[\
                                  \"md5\",\
                                  \"random\",\
                                  \"sha1\"\
                                  ]"),
        short = "-t"
    )]
    time: bool,
}
