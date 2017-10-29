extern crate cookie;
extern crate hyper;
extern crate hyper_serde;
extern crate mime;
extern crate serde;
extern crate time;

use cookie::Cookie;
use hyper::header::{ContentType, Headers};
use hyper::HttpVersion;
use hyper::Method;
use hyper::RawStatus;
use hyper::Uri;
use hyper_serde::{De, Ser, Serde};
use mime::Mime;
use serde::{Deserialize, Serialize};
use time::Tm;

fn is_supported<T>()
    where for<'de> De<T>: Deserialize<'de>,
          for<'a> Ser<'a, T>: Serialize,
          for <'de> Serde<T>: Deserialize<'de> + Serialize
{
}

#[test]
fn supported() {
    is_supported::<Cookie>();
    is_supported::<ContentType>();
    is_supported::<Headers>();
    is_supported::<HttpVersion>();
    is_supported::<Method>();
    is_supported::<Mime>();
    is_supported::<RawStatus>();
    is_supported::<Tm>();
    is_supported::<Uri>();
}
