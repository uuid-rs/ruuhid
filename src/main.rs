use std::str;

fn main() {
    let opts: RuuhidOpts = clap::Parser::parse();
}

#[derive(clap::Parser)]
enum RuuhidOpts {
   #[clap(aliases= &["g", "gen"])]
    Generate(RuuhidGenerateOpts),
    #[clap(aliases = &["p"])]
    Parse,
}

#[derive(clap::Parser)]
#[clap(about)]
struct RuuhidGenerateOpts {
    #[clap(subcommand)]
    version: RuuhidGenerateVersionOpts,
}

#[derive(clap::Parser)]
enum RuuhidGenerateVersionOpts {
    #[clap(aliases = &["0", "v0"])]
    Nil,
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
