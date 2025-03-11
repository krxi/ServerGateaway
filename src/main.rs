use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_extensible_rate_limit::backend::SimpleInputFunctionBuilder;
use actix_extensible_rate_limit::RateLimiter;
use actix_web::{App, HttpServer};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::http::StatusCode;


use simplelog::*;
use actix_governor::*;

use std::fs::File;         
use std::env;
use std::time;
use std::time::Duration;
use std::time::UNIX_EPOCH;

use dotenv::dotenv;

// implement mods
mod routes;

use routes::websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let log = true;

    /* # Setup for initiate project # */
    {
        // Creating log file

        if log {

             // get timestamp
            let time = time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Error occured when getting timestamp").as_secs();

            CombinedLogger::init(
                vec![
                    TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
                    WriteLogger::new(LevelFilter::Info, Config::default(), File::create(format!("logs/log_{0}.log",time)).unwrap()),
                ]
            ).unwrap();
        }
        
    
        info!("Booting app");
    
        // Reads and loads the environments from ".env"
        dotenv().ok();
    }
    /* # Setup ended # */

    let ip = env::var("IP").expect("You need to set 'IP' environment.");
    let port = env::var("PORT").expect("You need to set 'PORT' environment.");

    let address = format!("{}:{}", ip, port);
    info!("Server creating on {}",address);
    
    /* Anti ddos algorithms */

    /* Governor algorithm for secure [anti ddos / attack] */
    let governor_conf = GovernorConfigBuilder::default()
        .period(Duration::from_secs(3)) // refresh tokens every 3 second
        .burst_size(10) // limits requests
        .finish()
        .unwrap();

    /* Rate limit algorithm for secure [anti ddos / attack] */
    let backend = InMemoryBackend::builder().build();
 
    HttpServer::new(move || {

        /* Prepare ddos service */
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 29)
            .real_ip_key()
            .build();
        let middleware = RateLimiter::builder(backend.clone(), input)
            .add_headers()
            .build();
 
        // Route pages
   
        App::new()
            // Websocket
            .service(websocket::index)

            // Bridges
            .wrap(Governor::new(&governor_conf))
            .wrap(middleware)
            
    })
    .bind(address)?
    .run()
    .await
   
}
