# not_in_the_bible

A Twitter bot that searches for words from a tweet that are not in a book (for
e.g., the Bible).

A Twitter user must mention the bot's account under the tweet they want to
analyse. The bot only analyses the same tweet once.

This bot is multilingual: you can specify a book by language supported by
Twitter. For now messages are only localized in English and French.

## Build from source and configure

To build this project with [`cargo`](https://doc.rust-lang.org/cargo/):

```bash
cargo build --release
```

Then you must create a [PostgreSQL](https://www.postgresql.org/) user and
database (by default we will name it `not_in_the_bible`).

You also need to create a
[Twitter API v1.1 access](https://developer.twitter.com/en/portal/projects-and-apps)
to get Twitter API key and secret.

when you have done this, run this command to connect to your Twitter bot's
account with the API key and secret you got:

```bash
./target/release/generate_twitter_conf -k <YOUR_API_KEY> -s <YOUR_API_SECRET>
```

Copy [`config.yaml.example`](config.yaml.example) to create your configuration
file:

```bash
cp config.yaml.example config.yaml
```

In this file:

replace `username` and `password` in this line:

```yaml
db: postgres://username:password@localhost/not_in_the_bible
```

by the username and the password of the account you have created on your
PostreSQL database;

then replace these lines:

```yaml
twitter:
  api_key: <YOUR_API_KEY>
  api_secret: <YOUR_API_SECRET>
  token: <YOUR_ACCESS_TOKEN>
  token_secret: <YOUR_ACCESS_TOKEN_SECRET>
```

by the output of `generate_twitter_conf`.

### Deeper Twitter configuration

To avoid Twitter API's limits, you can edit two options in the `twitter` section
of the YAML configuration file:

- `updates_per_hour`: maximum number of statuses to send in one hour,
- `refresh_interval`: number of seconds between each refresh of the account's
  mentions.

## Sources configurations

To configure the books to look for words in, you can edit the `sources` section
of the YAML configuration file. By default it looks like that:

```yaml
sources:
  - path: examples/bible_en.txt
    name: the Bible
    lang: en
  - path: examples/bible_fr.txt
    name: la Bible
    lang: fr
```

As you can see you can specify multiple books, one by language supported by
Twitter. The bot will response by searching into the book corresponding to the
language of the status to analyse, or will default to the first source in the
list if no source are specified for this language.

For each source you need to specify:

- `path`: the path to its content, an UTF-8 encoded plain text file,
- `name`: the name of the book, that will appear in the responses the bot will
  send,
- `lang`: language tag for the book, corresponding to a language tag supported
  by Twitter.
