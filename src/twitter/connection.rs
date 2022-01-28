use std::{io::{self, Write, stdout}, collections::HashMap, borrow::Cow};

use log::info;
use oauth_client::{DefaultRequestBuilder, Token, Error};
use reqwest::blocking::Client;
use serde::{Deserialize, de::DeserializeOwned};

use crate::log_expect::LogExpect;

use super::{twitter_conf::TwitterConf, tweet::Tweet};

const TWITTER_API_REQUEST_TOKEN_URL: &str =
    "https://api.twitter.com/oauth/request_token";

const TWITTER_API_AUTHORIZE_URL: &str =
    "https://api.twitter.com/oauth/authorize";

const TWITTER_API_ACCESS_TOKEN_URL: &str =
    "https://api.twitter.com/oauth/access_token";

const TWITTER_API_MENTIONS_TIMELINE_URL: &str =
    "https://api.twitter.com/1.1/statuses/mentions_timeline.json";

const TWITTER_API_TWEET_SHOW_URL: &str =
    "https://api.twitter.com/1.1/statuses/show.json";

const TWITTER_API_STATUSES_UPDATE_URL: &str =
    "https://api.twitter.com/1.1/statuses/update.json";

const TWITTER_API_VERIFY_CREDENTIALS_URL: &str =
    "https://api.twitter.com/1.1/account/verify_credentials.json";

/// A connection to a Twitter account.
pub struct Connection<'a> {
    pub conf: TwitterConf,
    pub user_id: String,
    pub user_name: String,
    consumer: Token<'a>,
    token: Token<'a>,
}

/// Response from request_token
#[derive(Deserialize)]
struct RequestTokenResponse {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub oauth_callback_confirmed: bool,
}

/// Response from access_token
#[derive(Deserialize)]
struct AccessTokenResponse {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub user_id: String,
    pub screen_name: String,
}

/// Response from verify_credentials
#[derive(Deserialize)]
struct VerifyCredentials {
    pub id_str: String,
    pub screen_name: String,
}

impl <'a> Connection<'a> {

    /// Generates access tokens for given API key and secret.
    pub fn generate_access_token(api_key: &str, api_secret: &str) -> Token<'a> {
        let client = Client::builder()
            .build()
            .expect("Error building HTTP client");
        let consumer = Token::new(api_key.to_string(),
            api_secret.to_string());
        let request_token_response: RequestTokenResponse =
            serde_qs::from_str(&oauth_client::get::<DefaultRequestBuilder>(
            TWITTER_API_REQUEST_TOKEN_URL, &consumer, None, 
            None, &())
            .expect("Error getting request token"))
            .expect("Error deserializing request token response");
        let authorize_url = format!("{}?oauth_token={}",
            TWITTER_API_AUTHORIZE_URL, request_token_response.oauth_token);
        print!("Open {} and copy given code: ", authorize_url);
        stdout().flush().unwrap();
        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .expect("Error while reading stdin");
        let code = code.trim().to_string();
        let access_token_response: AccessTokenResponse = serde_qs::from_str(
            &client.post(TWITTER_API_ACCESS_TOKEN_URL)
            .query(&[
                ("oauth_token", request_token_response.oauth_token),
                ("oauth_verifier", code),
                ("oauth_verifier", api_key.to_string()),
            ])
            .send()
            .expect("Error sending POST oauth/access_token request")
            .text()
            .expect("Error getting access token text response"))
            .expect("Error deserializing access token response");
            Token::new(access_token_response.oauth_token,
                access_token_response.oauth_token_secret)
    }

    /// Creates a new connection to a Twitter account.
    pub fn init(conf: TwitterConf) -> Connection<'a> {
        let consumer = Token::new(conf.api_key.to_string(),
            conf.api_secret.to_string());
        let access_token = Token::new(conf.token.to_string(),
            conf.token_secret.to_string());
        let verify_credentials: VerifyCredentials = serde_json::from_str(
            &oauth_client::get::<DefaultRequestBuilder>(
                TWITTER_API_VERIFY_CREDENTIALS_URL,
                &consumer,
                Some(&access_token),
                None,
                &()
            ).log_expect("Error verifying user credentials"))
            .log_expect("Malformed JSON data");
        info!("Connection to Twitter account {} established",
            verify_credentials.screen_name);
        Connection {
            conf,
            consumer,
            token: access_token,
            user_id: verify_credentials.id_str,
            user_name: verify_credentials.screen_name,
        }
    }

    /// Send a GET request to the Twitter API.
    pub fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        query: HashMap<Cow<'a, str>, Cow<'a, str>>) -> Result<T, Error<DefaultRequestBuilder>> {
        match oauth_client::get::<DefaultRequestBuilder>(
            &url.to_string(), &self.consumer, Some(&self.token),
            Some(&query), &()) {
            Ok(string) => {
                let object: T = serde_json::from_str(&string)
                    .log_expect("Malformed JSON data");
                Ok(object)
            },
            Err(error) => Err(error),
        }
    }

    /// Send a POST request to the Twitter API.
    pub fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        body: HashMap<Cow<'a, str>, Cow<'a, str>>) -> Result<T, Error<DefaultRequestBuilder>> {
        match oauth_client::post::<DefaultRequestBuilder>(
            &url.to_string(), &self.consumer, Some(&self.token),
            Some(&body), &()) {
            Ok(string) => {
                let object: T = serde_json::from_str(&string)
                    .log_expect("Malformed JSON data");
                Ok(object)
            },
            Err(error) => Err(error),
        }
    }

    /// Returns mentions since a given id (latest first).
    pub fn mentions(&self, since: Option<&str>) -> Vec<Tweet> {
        let mut param: HashMap<Cow<'a, str>, Cow<'a, str>> = HashMap::new();
        if let Some(since) = since {
            param.insert(Cow::Borrowed("since_id"),
                Cow::Owned(since.to_string()));
        }
        param.insert(Cow::Borrowed("count"), Cow::Borrowed("200"));
        param.insert(Cow::Borrowed("tweet_mode"), Cow::Borrowed("extended"));
        self.get(TWITTER_API_MENTIONS_TIMELINE_URL, param)
            .unwrap_or(vec![])
    }

    /// Returns a tweet by its id.
    pub fn by_id(&self, id: &str) -> Result<Tweet, Error<DefaultRequestBuilder>>
    {
        let mut param: HashMap<Cow<'a, str>, Cow<'a, str>> = HashMap::new();
        param.insert(Cow::Borrowed("id"), Cow::Owned(id.to_string()));
        param.insert(Cow::Borrowed("include_entities"),
            Cow::Borrowed("false"));
        param.insert(Cow::Borrowed("tweet_mode"), Cow::Borrowed("extended"));
        self.get(TWITTER_API_TWEET_SHOW_URL, param)
    }

    /// Replies to tweet of given id (@username is automatically added to body).
    pub fn reply(&self, to_id: &str, to_users: &[&str], body: &str) ->
        Result<Tweet, Error<DefaultRequestBuilder>> {
        let mut param: HashMap<Cow<'a, str>, Cow<'a, str>> = HashMap::new();
        param.insert(Cow::Borrowed("status"),
            Cow::Owned(format!("{} {}", to_users
            .iter()
            .map(|u| format!("@{}", u))
            .collect::<Vec<String>>()
            .join(" "), body)));
        param.insert(Cow::Borrowed("in_reply_to_status_id"),
            Cow::Owned(to_id.to_string()));
        self.post(TWITTER_API_STATUSES_UPDATE_URL, param)
    }

    /// Quotes tweet of given id and given author.
    pub fn quote(&self, id: &str, author: &str, body: &'a str) ->
        Result<Tweet, Error<DefaultRequestBuilder>> {
        let mut param: HashMap<Cow<'a, str>, Cow<'a, str>> = HashMap::new();
        param.insert(Cow::Borrowed("status"), Cow::Borrowed(body));
        param.insert(Cow::Borrowed("attachment_url"), 
            Cow::Owned(format!("https://twitter.com/{}/status/{}", author,
            id)));
        self.post(TWITTER_API_STATUSES_UPDATE_URL, param)
    }

}