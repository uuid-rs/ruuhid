use std::str;

/// The `Case` enum is used to specify the case of the generated UUID.
#[derive(Clone, clap::Parser, clap::ValueEnum)]
pub enum Case {
    Lower,
    Upper,
}

/// The `Format` enum is used to specify the format of the generated UUID.
#[derive(Clone, clap::Parser, clap::ValueEnum)]
pub enum Format {
    Braced,
    Hyphenated,
    Simple,
    Urn,
}
