#[derive(Debug, Clone, Copy)]
pub struct Plugin {
    pub message_handler: Option<fn(String)>,
}
