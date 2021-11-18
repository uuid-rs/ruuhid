fn main() {
    let opts: RuuhidOpts = structopt::StructOpt::from_args();

    if let Some(subcommand) = &opts.subcommand {
    } else {
        println!("{}", uuid::Uuid::new_v4())
    }
}

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "ruuhid")]
struct RuuhidOpts {
    #[structopt(subcommand)]
    subcommand: Option<SubcommandOpts>,
}

#[derive(Debug, structopt::StructOpt)]
enum SubcommandOpts {
    #[structopt[aliases=&["gen"]]]
    Generate(RuuhidGenOpts),
}

#[derive(Debug, structopt::StructOpt)]
struct RuuhidGenOpts {
    #[structopt(subcommand)]
    subcommand: Option<RuuhidGenVersionOpts>,
}

#[derive(Debug, structopt::StructOpt)]
enum RuuhidGenVersionOpts {
    Nil,
    Mac,
    Dce,
    Md5,
    Random,
    Sha1,
}
