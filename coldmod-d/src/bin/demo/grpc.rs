use coldmod_msg::proto::{traces_client::TracesClient, Trace, TraceSrc, TraceSrcs};
use futures_util::stream;
use tonic::service::interceptor::InterceptedService;
use tonic::Request;
use tonic::Status;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
};

async fn traces_client(
) -> TracesClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, Status>>>
{
    let secure = !std::env::var("COLDMOD_INSECURE").map_or_else(|_| false, |v| v == "on");
    let grpc_host = std::env::var("COLDMOD_GRPC_HOST").expect("COLDMOD_GRPC_HOST not set");
    let url = if secure {
        format!("https://{}", grpc_host.clone())
    } else {
        format!("http://{}", grpc_host.clone())
    };
    let api_key = std::env::var("COLDMOD_API_KEY").expect("COLDMOD_API_KEY not set");

    let mut endpoint = Channel::from_shared(url).unwrap();

    if secure {
        let ca_path = std::env::var("COLDMOD_TLS_CA").expect("COLDMOD_TLS_CA not set");
        let pem = std::fs::read_to_string(ca_path).unwrap();
        let ca = Certificate::from_pem(pem);
        let tls = ClientTlsConfig::new()
            .ca_certificate(ca)
            .domain_name(&grpc_host);

        endpoint = endpoint.tls_config(tls).unwrap();
    }

    let channel = endpoint.connect().await.unwrap();

    let token: MetadataValue<_> = format!("Bearer {api_key}").parse().unwrap();

    let client = TracesClient::with_interceptor(channel, move |mut req: Request<()>| {
        if secure {
            req.metadata_mut().insert("authorization", token.clone());
        }
        Ok(req)
    });

    return client;
}

pub(crate) async fn trace(key: Option<String>, incr: Option<usize>) {
    let mut client = traces_client().await;

    let c = incr.unwrap_or(1);

    let mut traces: Vec<Trace> = Vec::new();

    let key = key.unwrap_or("a.fully_qualified_name.to.a.function".to_string());

    for _ in 0..c {
        traces.push(Trace {
            key: key.clone(),
            process_id: "0".into(),
            thread_id: "0".into(),
        });
    }

    match client.collect(stream::iter(traces)).await {
        Ok(_) => {
            println!("done.");
        }
        Err(e) => {
            eprintln!("failure sending traces: {:?}", e);
        }
    }
}

pub(crate) async fn set_trace_srcs_sample() {
    let mut client = traces_client().await;

    let key = "a.fully_qualified_name.to.a.function".to_string();

    let trace_srcs = TraceSrcs {
        trace_srcs: vec![TraceSrc { key }],
    };

    match client.set(trace_srcs).await {
        Ok(_) => {
            println!("done.");
        }
        Err(e) => {
            eprintln!("failure setting trace srcs sample: {:?}", e);
        }
    }
}
