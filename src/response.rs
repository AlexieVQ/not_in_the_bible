use crate::request::Request;

/// A response to a Request
pub struct Response {
    request: Request,
    body: String,
}

impl Response {

    /// Creates a new Response for given response with given body.
    pub fn new(request: Request, body: String) -> Response{
        Response { request, body }
    }

    /// The Request to which this response answers.
    pub fn response(&self) -> &Request {
        &self.request
    }

    /// The response's body (without usernames).
    pub fn body(&self) -> &String {
        &self.body
    }

}
