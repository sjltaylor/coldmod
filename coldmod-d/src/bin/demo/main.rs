// https://github.com/google/argh
use argh::FromArgs;

mod trace;

#[derive(FromArgs)]
/// utilities for working with coldmod data
struct Demo {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommand {
    Trace(Trace),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Simulate a trace
#[argh(subcommand, name = "trace")]
struct Trace {
    #[argh(positional)]
    /// tracing src key
    key: Option<String>,

    #[argh(option, short = 'n')]
    /// how many traces to simulated
    incr: Option<usize>,
}

#[tokio::main]
async fn main() {
    let demo: Demo = argh::from_env();
    match demo.subcommand {
        Subcommand::Trace(trace) => {
            trace::trace(trace.key, trace.incr).await;
        }
    }
}
