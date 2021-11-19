use std::str;

fn main() {
    let opts: RuuhidOpts = structopt::StructOpt::from_args();

    match opts.subcommand {
        None => {
            println!("{}", uuid::Uuid::new_v4())
        }
        Some(opts) => match opts {
            RuuhidSubOpts::Generate { subcommand } => match subcommand {
                None => {
                    println!("{}", uuid::Uuid::new_v4())
                }
                Some(opts) => match opts {
                    RuuhidGenVersionOpts::Nil(_) => {
                        println!("{}", uuid::Uuid::nil())
                    }
                    RuuhidGenVersionOpts::Mac => {}
                    RuuhidGenVersionOpts::Dce => {}
                    RuuhidGenVersionOpts::Md5 => {}
                    RuuhidGenVersionOpts::Random => {
                        println!("{}", uuid::Uuid::new_v4())
                    }
                    RuuhidGenVersionOpts::Sha1 => {}
                },
            },
        },
    }
}

#[derive(Clone, Debug, structopt::StructOpt)]
struct RuuhidOpts {
    #[structopt(subcommand)]
    subcommand: Option<RuuhidSubOpts>,
}

#[derive(Clone, Debug, structopt::StructOpt)]
enum RuuhidSubOpts {
    #[structopt[aliases=&["gen"]]]
    // Generate(RuuhidGenOpts),
    Generate {
        #[structopt(subcommand)]
        subcommand: Option<RuuhidGenVersionOpts>,
    },
}

#[derive(Clone, Debug, structopt::StructOpt)]
enum RuuhidGenVersionOpts {
    #[structopt(aliases=&["0"])]
    Nil(NilOpts),
    #[structopt(aliases=&["1"])]
    Mac,
    #[structopt(aliases=&["2"])]
    Dce,
    #[structopt(aliases=&["3"])]
    Md5,
    #[structopt(aliases=&["4"])]
    Random,
    #[structopt(aliases=&["5"])]
    Sha1,
}

#[derive(Clone, Debug, structopt::StructOpt)]
struct NilOpts {
    #[structopt(long, short)]
    format: FormatOpts,
}

#[derive(Clone, Debug, structopt::StructOpt)]
enum FormatOpts {
    Bytes,
    Guid,
    Hyphenated,
    Simple,
    Urn,
}

impl str::FromStr for FormatOpts {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
