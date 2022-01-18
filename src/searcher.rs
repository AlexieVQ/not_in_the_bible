use crate::{
    dictionary::Dictionary,
    response::Response,
    job_queue::JobQueue,
    request::Request
};

/// A routine that wait for requests and send responses to them.
fn run(request_queue: &mut impl JobQueue<Request>,
       response_queue: &mut impl JobQueue<Response>,
       dictionary: &impl Dictionary) {
    loop {
        let request = request_queue.take();
        let words = request.words();
        let absent_words = dictionary.absent_words(&words);
        let message: String = if words.len() == 1 {
            if absent_words.is_empty() {
                "This word is in the Bible".to_string()
            } else {
                "This word is not in the Bible".to_string()
            }
        } else if absent_words.is_empty() {
            "All these words are in the Bible".to_string()
        } else if absent_words.len() == words.len() {
            "None of these words are in the Bible".to_string()
        } else if absent_words.len() == 1 {
            format!("“{}” is not in the Bible", absent_words[0])
        } else {
            format!("“{}” are not in the Bible", absent_words.join("”, “"))
        };
        response_queue.submit(Response::new(&request, message));
    }
}