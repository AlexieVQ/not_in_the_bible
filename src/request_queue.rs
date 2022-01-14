use crate::request::Request;

/// Trait for request queues.
pub trait RequestQueue {

    /// Adds a request to this queue.
    fn submit(&mut self, request: Request);

    /// Takes a request from this queue (blocking).
    fn take(&mut self) -> Request;

}