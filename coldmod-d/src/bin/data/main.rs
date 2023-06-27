use std::str::FromStr;

// https://github.com/google/argh
use argh::FromArgs;

mod dump;
mod load;
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
    Load(Load),
    Trace(Trace),
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
/// Load data dumped from the store
#[argh(subcommand, name = "load")]
struct Load {
    #[argh(positional)]
    /// data key
    key: DataKey,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Simulate tracing
#[argh(subcommand, name = "trace")]
struct Trace {
    #[argh(positional)]
    /// source element key
    key: String,

    #[argh(option, short = 'n')]
    /// source element key
    incr: Option<usize>,
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
        Subcommand::Load(simulate) => match simulate.key {
            DataKey::SourceScan => {
                load::send_source_scan().await;
            }
            DataKey::TracingStream => {
                load::send_tracing_stream().await;
            }
        },
        Subcommand::Trace(trace) => {
            simulate::simulate_tracing(trace.key, trace.incr).await;
        }
    }
}
