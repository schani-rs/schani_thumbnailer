extern crate dotenv;
extern crate fern;
extern crate futures;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate mime;
extern crate schani_store_client;
extern crate tokio_core;

mod service;
mod web;

pub use web::webservice::ThumbnailWebService;
