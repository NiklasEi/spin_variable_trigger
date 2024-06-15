use anyhow::Context;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_variable_trigger(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let id = req
        .header("spin-path-match-id")
        .context("Failed to get variable from request path")?
        .as_str()
        .context("Failed to convert variable to str")?;
    println!("Got variable id='{id}'");
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())
}
