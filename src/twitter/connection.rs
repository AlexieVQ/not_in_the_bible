use std::{io::{self, Write, stdout}, collections::HashMap, borrow::Cow};

use oauth_client::{DefaultRequestBuilder, Token, Error};
use reqwest::blocking::Client;
use serde::{Deserialize, de::DeserializeOwned};

use super::twitter_conf::TwitterConf;

const TWITTER_API_REQUEST_TOKEN_URL: &str =
    "https://api.twitter.com/oauth/request_token";

const TWITTER_API_AUTHORIZE_URL: &str =
    "https://api.twitter.com/oauth/authorize";

const TWITTER_API_ACCESS_TOKEN_URL: &str =
    "https://api.twitter.com/oauth/access_token";

/// A connection to a Twitter account.
pub struct Connection<'a> {
    pub conf: &'a TwitterConf,
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

impl <'a> Connection<'a> {

    /// Creates a new connection to a Twitter account.
    pub fn init(conf: &'a TwitterConf) -> Connection<'a> {
        let client = Client::builder()
            .build()
            .expect("Error building HTTP client");
        let consumer = Token::new(&conf.api_key, &conf.api_secret);
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
                ("oauth_verifier", conf.api_key.to_string()),
            ])
            .send()
            .expect("Error sending POST oauth/access_token request")
            .text()
            .expect("Error getting access token text response"))
            .expect("Error deserializing access token response");
        let access_token = Token::new(access_token_response.oauth_token, access_token_response.oauth_token_secret);
        Connection { conf, consumer, token: access_token }
    }

    /// Send a GET request to the Twitter API.
    pub fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        body: HashMap<Cow<'a, str>, Cow<'a, str>>) -> Result<T, Error<DefaultRequestBuilder>> {
        match oauth_client::get::<DefaultRequestBuilder>(
            &url.to_string(), &self.consumer, Some(&self.token),
            Some(&body), &()) {
            Ok(string) => {
                let object: T = serde_json::from_str(&string)
                    .expect("Malformed JSON data");
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
                    .expect("Malformed JSON data");
                Ok(object)
            },
            Err(error) => Err(error),
        }
    }

}