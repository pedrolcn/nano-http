use actix_web::{web, App, HttpResponse, HttpServer};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "nano-http")]
struct Options {
    /// The URL path to mock
    #[structopt(long)]
    path: String,

    /// The HTTP method to mock
    #[structopt(short, long)]
    method: Option<String>,

    /// The response to be sent
    #[structopt(short, long)]
    respond_with: String,

    /// The response content-type
    #[structopt(short = "t", long, default_value = "application/json")]
    content_type: String,

    /// The port on which to listen for requests
    #[structopt(short, long, default_value = "3000")]
    port: u16,
}

struct State {
    response: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let Options {
        port,
        respond_with: response,
        method,
        content_type,
        path,
    } = Options::from_args();
    if std::env::var_os("RUST_lOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let handler = |state: web::Data<State>| {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(&state.response)
    };

    HttpServer::new(move || {
        App::new()
            .data(State {
                response: response.clone(),
            })
            .route(
                path.as_str(),
                match method.clone() {
                    None => web::route().to(handler),
                    Some(method) => match method.as_str() {
                        "GET" => web::get().to(handler),
                        "POST" => web::post().to(handler),
                        "PUT" => web::put().to(handler),
                        "PATCH" => web::patch().to(handler),
                        "DELETE" => web::delete().to(handler),
                        _ => panic!(format!("Method not supported {}", method)),
                    },
                },
            )
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
