use actix_web::{rt, web,Error, HttpRequest, HttpResponse, get};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;

use simplelog::*;

#[get("/ws_licence_control")]
pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    if let Some(val) = req.peer_addr() {
        // Logging IPs
        info!("{:?} Trying to connect websocket", val.ip());
    };

    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it (starts thread and continues to return OK for client)
    rt::spawn(async move {
        let level = 0;

        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}