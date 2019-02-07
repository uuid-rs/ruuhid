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
            conflicts_with_all = r#"&[
                                   "random",
                                   "sha1",
                                   "time"
                                   ]"#,
            requires_all = r#"&[
                             "name",
                             "namespace"
                             ]"#
        ),
        short = "-m"
    )]
    md5: bool,
    /// Generate hash-based uuid from this `name`
    #[structopt(
        long = "--name",
        raw(conflicts_with_all = r#"&[
                                      "random",
                                      "time"
                                      ]"#),
        short = "-N"
    )]
    name: Option<String>,
    /// Generate hash-based uuid in this `namespace`
    #[structopt(
        long = "--namespace",
        raw(conflicts_with_all = r#"&[
                                  "random",
                                  "time"
                                  ]"#),
        short = "-n"
    )]
    namespace: Option<crate::NamespaceUuid>,
    /// Generate random-based uuid
    #[structopt(
        long = "--random",
        raw(conflicts_with_all = r#"&[
                                  "md5",
                                  "sha1",
                                  "time"
                                  ]"#),
        short = "-r"
    )]
    random: bool,
    /// Generate sha1 hash
    #[structopt(
        long = "--sha1",
        raw(
            conflicts_with_all = r#"&[
                                  "md5",
                                  "random",
                                  "time"
                                  ]"#,
            requires_all = r#"&[
                                    "name",
                                    "namespace"
                                    ]"#
        ),
        short = "-s"
    )]
    sha1: bool,
    /// Generate time-based uuid
    #[structopt(
        long = "--time",
        raw(conflicts_with_all = r#"&[
                                  "md5",
                                  "random",
                                  "sha1"
                                  ]"#),
        short = "-t"
    )]
    time: bool,
}

impl CliOpts {
    /// The `--hex` or `-x` flag is present.
    pub fn hex(&self) -> bool {
        self.hex
    }

    /// The `--name` or `-N option is present.
    pub fn name(&self) -> Option<&str> {
        if let Some(name) = &self.name {
            Some(&name)
        } else {
            None
        }
    }

    /// The `--namespace` or `-n` option is present.
    pub fn namespace(&self) -> Option<crate::NamespaceUuid> {
        self.namespace
    }

    /// The `--md5` or `-m` flag is present.
    pub fn md5(&self) -> bool {
        self.md5
    }

    /// The `--random` or `-r` flag is present.
    pub fn random(&self) -> bool {
        self.random
    }

    /// The `--sha1` or `-s` flag is present.
    pub fn sha1(&self) -> bool {
        self.sha1
    }

    /// The `--time` or `-t` flag is present.
    pub fn time(&self) -> bool {
        self.time
    }
}
