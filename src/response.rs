use crate::request::Request;

/// A response to a Request
pub struct Response {
    request: &Request,
    body: String,
}

impl Response {

    /// Creates a new Response for given response with given body.
    fn new(request: &Request, body: String) -> Response{
        Response { request, body }
    }

    /// The Request to which this response answers.
    fn response(&self) -> &Request {
        self.request
    }

    /// The response's body (without usernames).
    fn body(&self) -> &String {
        &self.body
    }

}
