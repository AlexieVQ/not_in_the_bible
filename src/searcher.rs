use crate::{request_queue::RequestQueue, response_queue::{self, ResponseQueue}, dictionary::Dictionary};

/// A routine that wait for requests and send responses to them.
fn run(request_queue: &mut RequestQueue,
       response_queue: &mut ResponseQueue,
       dictionary: &impl Dictionary) {
    loop {
        let request = request_queue.take();
        let words = request.words();
        let absent_words = dictionary.absent_words(words);
        let message: String = if absent_words.is_empty() {
            "All these words are in the Bible"
        } else if absent_words.len() == words.len() {
            "None of these words are in the Bible"
        } else if absent_words.len() == 1 {
            format!("“{}” is not in the Bible", absent_words[0])
        } else {
            format!("“{}” are not in the Bible", absent_words.join("”, “"))
        };
        response_queue.submit(Response::new(&request, message));
    }
}