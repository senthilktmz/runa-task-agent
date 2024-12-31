use std::any::Any;
use std::sync::Arc;
use runautils::cipher_item;
use crate::orchestrator::server_util::ServerContext;

pub fn extract_payload_from_string(
    body: String,
    path: &'static str,
    server_context: &Arc<Box<dyn Any + Send + Sync>>,
) -> Result<(String, String), String> {
    if let Some(server_context) = server_context.downcast_ref::<ServerContext>() {
        let key = server_context.http_request_decrypt_key;

        // Decrypt the payload
        let result = cipher_item::get_decrypted_payload(body.clone(), key);

        match result {
            Ok(decrypted) => {
                return Ok((decrypted, body)); // Return decrypted payload and original body
            }
            Err(_) => {
                return Err(String::from("8c9a1eb4-a119-450c-967d-f53b0826e5e1"));
            }
        }
    } else {
        println!("Failed to downcast to ServerContext.");
        Err(String::from("71b3a699-4166-426d-aa24-eb59660a935e"))
    }
}