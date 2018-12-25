use std::error;

mod cli_opts;

fn main() -> Result<(), Box<error::Error>> {
    let opts = {
        use structopt::StructOpt;

        cli_opts::CliOpts::from_args()
    };

    // --md5
    if opts.md5() {
        // clap ensures that namespace url exists
        let name = opts.name().unwrap();
        let namespace = opts.namespace().unwrap();

        let uuid = uuid::Uuid::new_v3(&namespace, name.as_bytes());
        println!("{}", uuid);
        return Ok(());
    }

    // --sha1
    if opts.sha1() {
        // clap ensures that namespace url exists
        let name = opts.name().unwrap();
        let namespace = opts.namespace().unwrap();

        let uuid = uuid::Uuid::new_v5(&namespace, name.as_bytes());
        println!("{}", uuid);
        return Ok(());
    }

    // --time
    if opts.time() {
        let maybe_mac_address = mac_address::get_mac_address()?;

        // TODO: dont use the io::Error
        if let None = maybe_mac_address {
            return Err(Box::new(std::io::Error::from_raw_os_error(-1)));
        }

        let mac_address = maybe_mac_address.unwrap();
        let instant = std::time::SystemTime::now();
        let duration = instant.duration_since(std::time::UNIX_EPOCH)?;
        let mut context = uuid::v1::Context::new(0);
        let uuid = uuid::Uuid::new_v1(
            &context,
            duration.as_secs(),
            duration.subsec_nanos(),
            &mac_address.bytes(),
        )?;
        println!("{}", uuid);

        return Ok(());
    }

    // --random or fallback
    println!("{:x}", uuid::Uuid::new_v4());
    return Ok(());
}
