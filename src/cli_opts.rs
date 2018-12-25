#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "uuid")]
pub struct CliOpts {
    /// Interpret name as hex string
    #[structopt(long = "--hex", short = "-x")]
    hex: bool,
    /// Generate md5 hash
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
    /// Generate hash-based uuid from this `name`
    #[structopt(long = "--name", short = "-N")]
    name: Option<String>,
    /// Generate hash-based uuid in this `namespace`
    #[structopt(long = "--namespace", short = "-n")]
    namespace: Option<uuid::Uuid>,
    /// Generate random-based uuid
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
    /// Generate sha1 hash
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
    /// Generate time-based uuid
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

impl CliOpts {
    pub fn hex(&self) -> bool {
        self.hex
    }

    pub fn name(&self) -> Option<&str> {
        if let Some(name) = &self.name {
            Some(&name)
        } else {
            None
        }
    }

    pub fn namespace(&self) -> Option<uuid::Uuid> {
        self.namespace
    }

    pub fn md5(&self) -> bool {
        self.md5
    }

    pub fn random(&self) -> bool {
        self.random
    }

    pub fn sha1(&self) -> bool {
        self.sha1
    }

    pub fn time(&self) -> bool {
        self.time
    }
}
