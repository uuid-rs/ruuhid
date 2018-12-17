mod cli_opts;

fn main() {
    use structopt::StructOpt;

    let opt = cli_opts::CliOpts::from_args();
    println!("{:#?}", opt);

    println!("{:x}", uuid::Uuid::new_v4())
}
