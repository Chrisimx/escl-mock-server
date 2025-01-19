mod cli;
mod escl_server;

use actix_web::{web, App, HttpServer};

struct AppState {
    scanner_caps: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = cli::parse_cli();

    println!("Configuration: {args}");

    let scanner_caps = match args.scanner_caps_file {
        Some(file) => std::fs::read_to_string(file).expect("Couldn't read specified file"),
        None => include_str!("../res/default_scanner_caps.xml").to_owned(),
    };

    let app_data = web::Data::new(AppState {
        scanner_caps
    });

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::scope("/eSCL")
                .service(escl_server::scanner_capabilities)
                .service(escl_server::scan_job)
                .service(escl_server::next_doc)
            )
            .default_service(web::route().to(escl_server::not_found))
    })
    .bind((args.binding_address, args.port))
    .expect("Couldn't create HTTP server")
    .run();

    http_server.await
}
