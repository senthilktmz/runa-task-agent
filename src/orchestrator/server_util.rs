use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ServerContext<'a> {
    pub http_request_decrypt_key: &'a [u8; 32],
    pub state_storage_map: HashMap<String, Arc<Box<dyn Any + Send + Sync>>>,
    pub server_execution_instance_uuid: String,
}