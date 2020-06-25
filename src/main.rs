use femme;
use structopt::StructOpt;
use tide::{http::Method, Body, Request, Response};

#[derive(Debug, StructOpt, Clone)]
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
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Options = Options::from_args();
    femme::start(tide::log::Level::Debug.to_level_filter()).unwrap();

    let mut app = tide::with_state(args.clone());
    let mut route = app.at(args.path.as_str());

    let handler = |req: Request<Options>| async move {
        let state = req.state();

        let mut res = Response::new(200);
        res.set_body(Body::from_string(state.respond_with.clone()));
        res.set_content_type(tide::http::mime::JSON);
        Ok(res)
    };

    match args.method {
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

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
