use crate::opts::common;

/// Version of the UUID to generate.
#[derive(clap::Parser)]
pub enum VersionOpts {
    /// Generate a `nil` UUID.
    #[clap(aliases = &["0", "v0"], about)]
    Nil {
        #[clap(short, long, default_value = "lower")]
        case: common::Case,
        #[clap(short, long, default_value = "hyphenated")]
        format: common::Format,
        #[clap(short, long, default_value = "1")]
        number: usize,
    },
    /// Generate a `MAC` UUID.
    ///
    /// Currently not supported.
    #[clap(aliases = &["1", "v1"])]
    Mac,
    /// Generate a `DCE` UUID.
    ///
    /// Currently not supported.
    #[clap(aliases = &["2", "v2"])]
    Dce,
    /// Generate a `MD5` UUID.
    ///
    /// Currently not supported.
    #[clap(aliases = &["3", "v3"])]
    Md5,
    /// Generate a `random` UUID.
    #[clap(aliases = &["4", "v4"])]
    Random {
        #[clap(short, long, default_value = "lower")]
        case: common::Case,
        #[clap(short, long, default_value = "hyphenated")]
        format: common::Format,
        #[clap(short, long, default_value = "1")]
        number: usize,
    },
    /// Generate a `SHA1` UUID.
    ///
    /// Currently not supported.
    #[clap(aliases = &["5", "v5"])]
    Sha1,
}
