use std::str;
use uuid;

fn main() {
    let opts: Opts = clap::Parser::parse();

    match opts {
        Opts::Generate(opts) => match opts.version {
            GenerateVersionOpts::Nil(opts) => {
                (0..opts.number).for_each(|_| {
                    let uuid = uuid::Uuid::nil();
                    let format = &opts.format;
                    let case = &opts.case;

                    match case {
                        CaseOpts::Lower => match format {
                            FormatOpts::Braced => println!("{:x}", uuid.braced()),
                            FormatOpts::Hyphenated => println!("{:x}", uuid.hyphenated()),
                            FormatOpts::Simple => println!("{:x}", uuid.simple()),
                            FormatOpts::Urn => println!("{:x}", uuid.urn()),
                        },
                        CaseOpts::Upper => match format {
                            FormatOpts::Braced => println!("{:X}", uuid.braced()),
                            FormatOpts::Hyphenated => println!("{:X}", uuid.hyphenated()),
                            FormatOpts::Simple => println!("{:X}", uuid.simple()),
                            FormatOpts::Urn => println!("{:X}", uuid.urn()),
                        },
                    }
                });
            }
            GenerateVersionOpts::Mac => {}
            GenerateVersionOpts::Dce => {}
            GenerateVersionOpts::Md5 => {}
            GenerateVersionOpts::Random => {}
            GenerateVersionOpts::Sha1 => {}
        },
        Opts::Parse => {}
    }
}

#[derive(clap::Parser)]
#[clap(about, author, version)]
enum Opts {
    #[clap(aliases= &["g", "gen"])]
    Generate(GenerateOpts),
    #[clap(aliases = &["p"])]
    Parse,
}

#[derive(clap::Parser)]
#[clap(about)]
struct GenerateOpts {
    #[clap(subcommand)]
    version: GenerateVersionOpts,
}

#[derive(clap::Parser)]
enum GenerateVersionOpts {
    #[clap(aliases = &["0", "v0"])]
    Nil(GenerateNilOpts),
    #[clap(aliases = &["1", "v1"])]
    Mac,
    #[clap(aliases = &["2", "v2"])]
    Dce,
    #[clap(aliases = &["3", "v3"])]
    Md5,
    #[clap(aliases = &["4", "v4"])]
    Random,
    #[clap(aliases = &["5", "v5"])]
    Sha1,
}

#[derive(clap::Parser)]
struct GenerateNilOpts {
    #[clap(short, long, default_value = "lowercase")]
    case: CaseOpts,
    #[clap(short, long, default_value = "1")]
    number: usize,
    #[clap(short, long, default_value = "hyphenated")]
    format: FormatOpts,
}

#[derive(Clone, clap::Parser)]
enum FormatOpts {
    Braced,
    Hyphenated,
    Simple,
    Urn,
}

impl str::FromStr for FormatOpts {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "braced" | "b" => Ok(FormatOpts::Braced),
            "hyphenated" | "hypen" | "h" => Ok(FormatOpts::Hyphenated),
            "simple" | "s" => Ok(FormatOpts::Simple),
            "urn" | "u" => Ok(FormatOpts::Urn),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

#[derive(Clone, clap::Parser)]
enum CaseOpts {
    Lower,
    Upper,
}

impl str::FromStr for CaseOpts {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lowercase" | "lower" | "l" => Ok(CaseOpts::Lower),
            "uppercase" | "upper" | "u" => Ok(CaseOpts::Upper),
            _ => Err(format!("Invalid case: {}", s)),
        }
    }
}
