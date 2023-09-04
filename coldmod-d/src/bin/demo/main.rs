// https://github.com/google/argh
use argh::FromArgs;

mod grpc;

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
    SetTraceSrcsSample(SetTraceSrcsSample),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Simulate a trace
#[argh(subcommand, name = "trace")]
struct Trace {
    #[argh(positional)]
    /// tracing src key
    key: Option<String>,

    #[argh(option, short = 'n')]
    /// how many traces to simulate
    incr: Option<usize>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Set trace srcs to a sample
#[argh(subcommand, name = "set-trace-srcs-sample")]
struct SetTraceSrcsSample {
    #[argh(switch)]
    /// confirm this destructive action
    confirm: bool,
}

#[tokio::main]
async fn main() {
    let demo: Demo = argh::from_env();
    match demo.subcommand {
        Subcommand::Trace(trace) => {
            grpc::trace(trace.key, trace.incr).await;
        }
        Subcommand::SetTraceSrcsSample(set_trace_srcs_sample) => {
            if set_trace_srcs_sample.confirm {
                grpc::set_trace_srcs_sample().await;
            } else {
                println!(
                    "--confirm that you want to set trace srcs - this is a destructive action."
                );
            }
        }
    }
}
