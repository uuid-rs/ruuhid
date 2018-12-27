use std::error;

mod cli_opts;
mod core_support;

macro_rules! hashed {
    (v3, $opts: expr) => {
        {
            let name = $opts.name().unwrap();
            let namespace: uuid::Uuid = $opts.namespace().unwrap().into();

            uuid::Uuid::new_v3(&namespace, name.as_bytes())
        }
    };

    (v5, $opts: expr) => {
        {
            let name = $opts.name().unwrap();
            let namespace: uuid::Uuid = $opts.namespace().unwrap().into();

            uuid::Uuid::new_v5(&namespace, name.as_bytes())
        }
    };
}

#[derive(Clone, Copy, Debug)]
struct NamespaceUuid(uuid::Uuid);

impl NamespaceUuid {
    pub fn new(uuid: uuid::Uuid) -> Self {
        NamespaceUuid(uuid)
    }

    pub fn parse(string: &str) -> Result<Self, NamespaceUuidError> {
        if string.starts_with('@') {
            match string {
                "@dns" => Ok(NamespaceUuid::new(uuid::Uuid::NAMESPACE_DNS)),
                "@oid" => Ok(NamespaceUuid::new(uuid::Uuid::NAMESPACE_OID)),
                "@url" => Ok(NamespaceUuid::new(uuid::Uuid::NAMESPACE_URL)),
                "@x500" => Ok(NamespaceUuid::new(uuid::Uuid::NAMESPACE_X500)),
                rest => Err(NamespaceUuidError::WellKnownUuid(rest.to_string()))
            }
        } else {
            let maybe_uuid = uuid::Uuid::parse_str(string);

            match maybe_uuid {
                Ok(uuid) => Ok(NamespaceUuid::new(uuid)),
                Err(err) => Err(NamespaceUuidError::Uuid(err))
            }
        }
    }
}

#[derive(Debug)]
enum NamespaceUuidError {
    WellKnownUuid(String),
    Uuid(uuid::parser::ParseError)
}

fn main() -> Result<(), Box<error::Error>> {
    let opts = {
        use structopt::StructOpt;

        cli_opts::CliOpts::from_args()
    };

    // --md5
    if opts.md5() {
        let uuid = hashed!(v3, opts);
        println!("{}", uuid);
        return Ok(())
    }

    // --sha1
    if opts.sha1() {
        let uuid = hashed!(v5, opts);
        println!("{}", uuid);
        return Ok(())
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
        let context = uuid::v1::Context::new(0);
        let uuid = uuid::Uuid::new_v1(
            &context,
            duration.as_secs(),
            duration.subsec_nanos(),
            &mac_address.bytes(),
        )?;
        println!("{}", uuid);

        return Ok(())
    }

    // --random or fallback
    println!("{:x}", uuid::Uuid::new_v4());
    return Ok(())
}
