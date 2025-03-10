use actix_web::{App, HttpServer};

use simplelog::*;
use std::fs::File;         

use std::env;
use std::time;
use std::time::UNIX_EPOCH;

use dotenv::dotenv;

// implement mods
mod routes;

use routes::websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    /* # Setup for initiate project # */
    {
        // get timestamp
        let time = time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Error occured when getting timestamp").as_secs();
    
        // Creating log file
        CombinedLogger::init(
            vec![
                TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
                WriteLogger::new(LevelFilter::Info, Config::default(), File::create(format!("logs/log_{0}.log",time)).unwrap()),
            ]
        ).unwrap();
    
        info!("Booting app");
    
        // Reads and loads the environments from ".env"
        dotenv().ok();
    }
    /* # Setup ended # */

    let ip = env::var("IP").expect("You need to set 'IP' environment.");
    let port = env::var("PORT").expect("You need to set 'PORT' environment.");

    let address = format!("{}:{}", ip, port);
    info!("Server creating on {}",address);
  
    HttpServer::new(|| {
        // Route pages
       
        App::new()
            // Websocket
            .service(websocket::index)
    })
    .bind(address)?
    .run()
    .await
   
}
