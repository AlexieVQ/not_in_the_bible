use chrono::NaiveDateTime;
use diesel::Queryable;

use crate::request::Request;
use crate::schema::responses;

/// A response to a Request
#[derive(Queryable, Insertable)]
pub struct Response {
    pub id: String,
    pub body: String,
    pub user: String,
    pub date: NaiveDateTime,
    pub op_id: String,
    pub op_author: String,
}

impl Response {

    /// Creates a new Response for given response with given body.
    pub fn new(request: &Request, body: String) -> Response{
        Response {
            id: request.id.to_string(),
            body,
            user: request.user.to_string(),
            date: request.date,
            op_id: request.op_id.to_string(),
            op_author: request.op_author.to_string(),
        }
    }

}
