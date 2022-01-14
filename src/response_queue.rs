use crate::response::Response;

/// Trait for response queues.
pub trait ResponseQueue {

    /// Adds a response to this queue.
    fn submit(&mut self, response: Response);

    /// Takes a response from this queue (blocking).
    fn take(&mut self) -> Response;

}