use {
    hyper::{
        service::{make_service_fn, service_fn},
        Body, Client, Request, Response, Server, Uri,
    },
    std::net::SocketAddr,
};

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Got request at {:?}", req.uri());

    // Redirect to the Rust homepage
    let url_str = "http://www.rust-lang.org/en-US/";
    let url = url_str.parse::<Uri>().expect("failed to parse URL");
    let res = Client::new().get(url).await?;

    println!("request finished-- returning response");
    Ok(res)
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    // Create server
    let serve_future = Server::bind(&addr)
        // Serve requests using our `async serve_req` function.
        // `serve` takes a closure which returns a type implementing the
        // `Service` trait. `service_fn` returns a value implementing the
        // `Service` trait, and accepts a closure which goes from request
        // to a future of the response.
        .serve(make_service_fn(|_| async {
            {
                Ok::<_, hyper::Error>(service_fn(serve_req))
            }
        }));

    // Wait for server to complete serving or exit
    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run_server(addr).await;
}
