use not_in_the_bible::twitter::connection::Connection;
use oauth_client::Token;
use rustop::opts;

fn main() {
    let (args, _) = opts! {
        synopsis "Request Twitter API access tokens";
        opt key: String, desc: "Twitter API key";
        opt secret: String, desc: "Twitter API secret";
    }.parse_or_exit();

    let consumer = Token::new(&args.key, &args.secret);
    let access_token = Connection::generate_access_token(&consumer.key,
        &consumer.secret);

    println!("twitter:");
    println!("  api_key: {}", &args.key);
    println!("  api_secret: {}", &args.secret);
    println!("  token: {}", &access_token.key);
    println!("  token_secret: {}", &access_token.secret);
}