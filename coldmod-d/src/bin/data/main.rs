use std::str::FromStr;

// https://github.com/google/argh
use argh::FromArgs;

mod dump;
mod simulate;

#[derive(FromArgs)]
/// utilities for working with coldmod data
struct Data {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommand {
    Dump(Dump),
    Simulate(Simulate),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Dump data
#[argh(subcommand, name = "dump")]
struct Dump {
    #[argh(positional)]
    /// dataset key
    key: DataKey,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Simulate a client sending data
#[argh(subcommand, name = "simulate")]
struct Simulate {
    #[argh(positional)]
    /// dataset key
    key: DataKey,
}

#[derive(Eq, PartialEq, Debug)]
enum DataKey {
    SourceScan,
    TracingStream,
}

impl FromStr for DataKey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "source-scan" => Ok(DataKey::SourceScan),
            "tracing-stream" => Ok(DataKey::TracingStream),
            _ => Err(format!("unknown dumpable key: {}", s)),
        }
    }
}

#[tokio::main]
async fn main() {
    let data: Data = argh::from_env();
    match data.subcommand {
        Subcommand::Dump(dump) => match dump.key {
            DataKey::SourceScan => {
                dump::dump_source_scan().await;
            }
            DataKey::TracingStream => {
                dump::dump_tracing_stream().await;
            }
        },
        Subcommand::Simulate(simulate) => match simulate.key {
            DataKey::SourceScan => {
                simulate::send_source_scan().await;
            }
            DataKey::TracingStream => {
                simulate::send_tracing_stream().await;
            }
        },
    }
}
