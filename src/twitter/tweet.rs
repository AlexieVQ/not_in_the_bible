use serde::{Deserialize, Serialize};

/// A Twitter status
#[derive(Deserialize)]
pub struct Tweet {
    pub id_str: String,
    pub created_at: String,
    pub text: String,
    pub in_reply_to_status_id_str: Option<String>,
    pub in_reply_to_screen_name: Option<String>,
    pub in_reply_to_user_id_str: Option<String>,
    pub user: User,
    pub is_quote_status: bool,
    pub quoted_status: Option<Box<Tweet>>,
    pub lang: Option<String>,
}

/// A Twitter user profile
#[derive(Deserialize)]
pub struct User {
    pub screen_name: String,
}

/// A Twitter draft
#[derive(Serialize)]
pub struct Draft {
    pub status: String,
    pub in_reply_to_status_id: String,
}