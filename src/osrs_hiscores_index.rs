mod osrs;

use lambda_http::{run, service_fn, Error, IntoResponse, RequestPayloadExt};

use serde::{Deserialize, Serialize};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    page: usize,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    status_code: i32,
    items: Option<osrs::HiscoresIndex>,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(request: lambda_http::Request) -> Result<impl IntoResponse, Error> {
    let request: Request = request.payload().unwrap().unwrap();
    // Prepare the response
    let resp = Response {
        status_code: 200,
        items: osrs::hiscores_index(request.page).await?,
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(serde_json::to_string(&resp).unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
