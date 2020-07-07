use femme;
use structopt::StructOpt;
use tide::{http::Method, Body, Request, Response};

#[derive(Debug, StructOpt)]
#[structopt(name = "nano-http")]
struct Options {
    /// The URL path to mock
    #[structopt(long)]
    path: String,

    /// The HTTP method to mock
    #[structopt(short, long)]
    method: Option<Method>,

    /// The response to be sent
    #[structopt(short, long)]
    respond_with: String,

    /// The response content-type
    #[structopt(short = "t", long, default_value = "application/json")]
    content_type: String,

    /// The port on which to listen for requests
    #[structopt(short, long, default_value = "3000")]
    port: u16,

    /// The respose status code
    #[structopt(long, default_value = "200")]
    status: u16,
}

struct State {
    response: String,
    status: u16,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let Options {
        port,
        respond_with: response,
        method,
        content_type,
        path,
        status,
    } = Options::from_args();
    femme::with_level(tide::log::Level::Debug.to_level_filter());

    let mut app = tide::with_state(State { response, status });
    let mut route = app.at(path.as_str());

    let handler = |req: Request<State>| async move {
        let state = req.state();

        let mut res = Response::new(state.status);
        res.set_body(Body::from_string(state.response.clone()));
        res.set_content_type(tide::http::mime::JSON);
        Ok(res)
    };

    match method {
        None => route.all(handler),
        Some(method) => match method {
            Method::Get => route.get(handler),
            Method::Post => route.post(handler),
            Method::Put => route.put(handler),
            Method::Patch => route.patch(handler),
            Method::Delete => route.delete(handler),
            _ => panic!(format!("Method not supported {}", method)),
        },
    };

    app.listen(format!("127.0.0.1:{}", port)).await?;
    Ok(())
}
