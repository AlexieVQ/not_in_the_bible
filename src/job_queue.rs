/// Trait for job queues (request, responses).
pub trait JobQueue<J> {

    /// Adds a job to this queue.
    fn submit(&mut self, request: J);

    /// Takes a job from this queue (blocking).
    fn take(&mut self) -> J;

}