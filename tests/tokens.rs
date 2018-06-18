extern crate cookie;
extern crate hyper;
extern crate hyperx;
extern crate hyper_serde;
extern crate serde;
extern crate serde_test;
extern crate time;

use cookie::Cookie;
use hyperx::header::{ContentType, Headers};
use hyper::Method;
use hyper::Uri;
use hyper_serde::{De, Ser, deserialize};
use serde::Deserialize;
use serde_test::{Deserializer, Token, assert_ser_tokens};
use std::fmt::Debug;
use time::Duration;

#[test]
fn test_content_type() {
    let content_type = ContentType("Application/Json".parse().unwrap());
    let tokens = &[Token::Str("application/json")];

    assert_ser_tokens(&Ser::new(&content_type), tokens);
    assert_de_tokens(&content_type, tokens);
}

#[test]
fn test_cookie() {
    let cookie = Cookie::build("Hello", "World!")
        .max_age(Duration::seconds(42))
        .domain("servo.org")
        .path("/")
        .secure(true)
        .http_only(false)
        .finish();

    let tokens = &[Token::Str("Hello=World!; Secure; Path=/; \
                               Domain=servo.org; Max-Age=42")];

    assert_ser_tokens(&Ser::new(&cookie), tokens);
    assert_de_tokens(&cookie, tokens);
}

#[test]
fn test_headers_empty() {
    let headers = Headers::new();

    let tokens = &[Token::Map { len: Some(0) }, Token::MapEnd];

    assert_ser_tokens(&Ser::new(&headers), tokens);
    assert_de_tokens(&headers, tokens);
}

#[test]
fn test_headers_not_empty() {
    use hyperx::header::Host;

    let mut headers = Headers::new();
    headers.set(Host::new(
        "baguette",
        None
    ));

    // In Hyper 0.9, Headers is internally a HashMap and thus testing this
    // with multiple headers is non-deterministic.

    let tokens = &[Token::Map{ len: Some(1) },
                   Token::Str("Host"),
                   Token::Seq{ len: Some(1) },
                   Token::Bytes(b"baguette"),
                   Token::SeqEnd,
                   Token::MapEnd];

    assert_ser_tokens(&Ser::new(&headers), tokens);
    assert_de_tokens(&headers, tokens);

    let pretty = &[Token::Map{ len: Some(1) },
                   Token::Str("Host"),
                   Token::Seq{ len: Some(1) },
                   Token::Str("baguette"),
                   Token::SeqEnd,
                   Token::MapEnd];

    assert_ser_tokens(&Ser::new_pretty(&headers), pretty);
    assert_de_tokens(&headers, pretty);
}

#[test]
fn test_method() {
    let method = Method::PUT;
    let tokens = &[Token::Str("PUT")];

    assert_ser_tokens(&Ser::new(&method), tokens);
    assert_de_tokens(&method, tokens);
}

#[test]
fn test_tm() {
    use time::strptime;

    let time = strptime("2017-02-22T12:03:31Z", "%Y-%m-%dT%H:%M:%SZ").unwrap();
    let tokens = &[Token::Str("2017-02-22T12:03:31Z")];

    assert_ser_tokens(&Ser::new(&time), tokens);
    assert_de_tokens(&time, tokens);
}

#[test]
fn test_uri() {
    use std::str::FromStr;

    // Note that fragment is not serialized / deserialized
    let uri_string = "abc://username:password@example.com:123/path/data?key=value&key2=value2";
    let uri = Uri::from_str(uri_string).unwrap();
    let tokens = &[Token::Str(uri_string)];

    assert_ser_tokens(&Ser::new(&uri), tokens);
    assert_de_tokens(&uri, tokens);
}

pub fn assert_de_tokens<T>(value: &T, tokens: &[Token])
    where T: Debug + PartialEq,
          for<'de> De<T>: Deserialize<'de>,
{
    let mut deserializer = Deserializer::new(&tokens);
    let v = deserialize::<T, _>(&mut deserializer).unwrap();
    assert_eq!(&v, value);
    assert_eq!(deserializer.next_token_opt(), None);
}
