#[derive(Debug, Clone, Copy)]
pub struct Plugin {
    pub message_handler: Option<*mut fn(String)>,
}
