use crate::opts::{Opts, VersionOpts};
use uuid;
use crate::opts::common as common_opts;

mod execute;
mod opts;

fn main() {
    let opts = Opts::parse();

    match opts {
        Opts::Generate { version } => match version {
            VersionOpts::Nil {
                case,
                format,
                number,
            } => {
                (0..number).for_each(|_| {
                    let uuid = uuid::Uuid::nil();

                    match case {
                        common_opts::Case::Lower => match format {
                            common_opts::Format::Braced => println!("{:x}", uuid.braced()),
                            common_opts::Format::Hyphenated => println!("{:x}", uuid.hyphenated()),
                            common_opts::Format::Simple => println!("{:x}", uuid.simple()),
                            common_opts::Format::Urn => println!("{:x}", uuid.urn()),
                        },
                        common_opts::Case::Upper => match format {
                            common_opts::Format::Braced => println!("{:X}", uuid.braced()),
                            common_opts::Format::Hyphenated => println!("{:X}", uuid.hyphenated()),
                            common_opts::Format::Simple => println!("{:X}", uuid.simple()),
                            common_opts::Format::Urn => {
                                panic!("UUID URN format is not supported in uppercase")
                            }
                        },
                    }
                });
            }
            VersionOpts::Mac => todo!("MAC UUID generation is not supported"),
            VersionOpts::Dce => todo!("DCE UUID generation is not supported"),
            VersionOpts::Md5 => todo!("MD5 UUID generation is not supported"),
            VersionOpts::Random {
                case,
                format,
                number,
            } => {
                (0..number).for_each(|_| {
                    let uuid = uuid::Uuid::new_v4();

                    match case {
                        common_opts::Case::Lower => match format {
                            common_opts::Format::Braced => println!("{:x}", uuid.braced()),
                            common_opts::Format::Hyphenated => println!("{:x}", uuid.hyphenated()),
                            common_opts::Format::Simple => println!("{:x}", uuid.simple()),
                            common_opts::Format::Urn => println!("{:x}", uuid.urn()),
                        },
                        common_opts::Case::Upper => match format {
                            common_opts::Format::Braced => println!("{:X}", uuid.braced()),
                            common_opts::Format::Hyphenated => println!("{:X}", uuid.hyphenated()),
                            common_opts::Format::Simple => println!("{:X}", uuid.simple()),
                            common_opts::Format::Urn => {
                                panic!("UUID URN format is not supported in uppercase")
                            }
                        },
                    }
                });
            }
            VersionOpts::Sha1 => todo!("SHA1 UUID generation is not supported"),
        },
        Opts::Parse => {}
    }
}
