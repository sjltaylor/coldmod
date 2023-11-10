# About

## Motivation

Cold code is code which is never run in production.

Lot's of codebases have cold code. It's often the result of refactoring, or a change in requirements, or a change in the environment.

It's also, pure waste and risk. Cold code is still built and deployed, its a maintenance overhead and a potential attack surface -- it's noise.

Being able to find cold code is a useful for developers and managers. It can help raise questions and understand the codebase. At the very least, when you're new to a project, it would be good to know if code is cold so you don't spend time tring to understanding what it does.

The volume of cold code could also be used as a quality signal for engineering management -- hypothesis being that cold code volume increase is a leading indicator for deliver performance decrease.

So this project is an experiment in building a toolkit to find _and remove_ cold code. If stops short of "self-cleaning" - removal is a development decision.

The implementation is guided by the following principles:
- the data needs to be accurate
- it needs to be able to collect enough data to be a meaningful sample
    - not _all_ servers
    - not deployed indefinitely
- we need to verify the utility of the whole workflow
- we need to be able to experiment with the data for other uses

## Implementation Notes

Only Python 3 is supported, and only one application can be traced at a time. tracing accross multiple services/microservices wasn't the initial use case, but might be useful.

With this initial version, we are able to explore the utility if monitoring data in the development workflow. This is why the workflow spans from production to development as far as is practical.


### coldmod-d

The server is implemented Rust with the tokio runtime. It was always going to have at least two interfaces, one for gRPC and one for websockets, but picking a technogies used for each was a journey. So it's a hybrid server, with a gRPC server based on tonic, and a websocket server based on axum. These are separate servers, even though they're both based on hyper. gRPC has been useful for sharing types and interfaces - and useful for out of the box in-datacenter performance. For websocket communication, we use flexbuffers.

Storage is provided by Redis. The goal was to create something that was good enough to endure production workloads. The major limitation is that it's all in memory, and the Redis stream of tracing events has infinite retention. Since we're not intending to deploy the server long-term, this can be worked around with a large enough instance and/or the introduction of stream windowing.

Depending on what obserabvability tooling you have, you might be able to collect tracing data without needing the Redis Stream, and create a variation of `coldmod-d` that integrates with other tooling.

### coldmod-web

The Web app is implemented with Leptos. The framework is great, but integrating with DOM APIs is painful. I was advised against building a Rust-wasm frontend, but I did it anyway to put pressure on learning about lifetimes. It's no so bad that `coldmod-web` will be replaced, but any new UIs will probably use a Typescript+React stack.


The interactions between the CLI and Web App when in `connect`ed mode, are a bit like those of a language server and IDE. This ought to be researched for future development. More functionality in this mode will put pressure on the robustness and performance of CLI implementation - it would be a shame for this to be language-specific.


### coldmod-py

This library and CLI are key to the experiment. The workflow starts here with tracing, and ends with the CLI being use to apply mods. We use LibCST, supported by some functionality from Jedi for finding references to functions through a python project. Jedi makes it a bit slow, but we can easily reimplement the functionality we need in LibCST, should the design of first iteration be successful. If the design proves worthy of further investment, we can look at other options for parsing and codegen in a more language agnostic way. We might also consider separating the CLI and tracing libraries. We also need to gain confidence in functions being the most useful tracing source versus blocks, classes, modules etc - or custom trace-points.

Usage with async python has not been explored.
