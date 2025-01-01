use serde_json::{self, Value};
use std::any::Any;
use actix::{Actor, ActorContext, StreamHandler};
use actix_web::{HttpResponse};
use actix_web_actors::ws;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use runautils::actix_server_util::ServerStateStore;
use crate::orchestrator::payload_util::extract_payload_from_string;
use std::time::Duration;
use actix::AsyncContext;
use crate::orchestrator::run_task_set::process_run_task_set;

pub fn websocket_handler2(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
    server_context: Arc<Box<dyn Any + Send + Sync>>,
    server_state_store: Arc<Mutex<ServerStateStore>>,
) -> Pin<Box<dyn Future<Output = Result<HttpResponse, actix_web::Error>>>> {
    Box::pin(async move {
        println!("WebSocket handler invoked with server context and state store");
        ws::start(WebSocketActor {
            server_context,
            server_state_store,
        }, &req, stream)
    })
}

pub struct WebSocketActor {
    server_context: Arc<Box<dyn Any + Send + Sync>>,
    server_state_store: Arc<Mutex<ServerStateStore>>,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection started by the task executor");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket connection stopped");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received WebSocket message: {}", text);

                match extract_payload_from_string(text.parse().unwrap(), "", &self.server_context) {
                    Ok((decrypted_payload, original_body)) => {
                        match process_run_task_set(decrypted_payload, ctx) {
                           Ok(response) => {
                               println!("Processed response from the task executor");
                           } ,
                            Err(err) => {
                                println!("Error processing response from task executor: {}", err);
                            }
                        }
                    }
                    Err(err) => {
                        println!("Error in extract_payload: {}", err);
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => {
                println!("Ping received");
                ctx.pong(&msg);
            }
            Ok(ws::Message::Close(_)) => {
                println!("Client closed connection");
                ctx.text("Client closed connection");
                ctx.stop();
            }
            _ => println!("Other message type received"),
        }
    }
}


/// Function to process the JSON message
fn process_json_message(
    json_value: &Value,
    server_context: &Arc<Box<dyn Any + Send + Sync>>,
    server_state_store: &Arc<Mutex<ServerStateStore>>,
) -> Result<String, String> {

    println!("--------------------------------------{}", "");
    println!("{:#?}", json_value);
    println!("--------------------------------------{}", "");
    Ok(format!("Processed task: {}", "ok"))
}
