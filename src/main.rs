extern crate dotenv;
extern crate log;
extern crate schani_thumbnailer;

use std::env;

use dotenv::dotenv;
use schani_thumbnailer::ThumbnailWebService;

fn main() {
    dotenv().ok();
    let store_url = env::var("STORE_URL").expect("STORE_URL required");
    let web_service = ThumbnailWebService::new(&store_url);

    web_service.run();
}
