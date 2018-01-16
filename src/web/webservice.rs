use fern;
use gotham::start;

use std::io;

use super::router::build_app_router;

use log::LevelFilter;

pub struct ThumbnailWebService<'a> {
    store_url: &'a str,
}

impl<'a> ThumbnailWebService<'a> {
    pub fn new(store_url: &'a str) -> Self {
        ThumbnailWebService {
            store_url: store_url,
        }
    }

    fn set_logging(&self) {
        fern::Dispatch::new()
            .level(LevelFilter::Info)
            .chain(io::stdout())
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}][{}]{}",
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .apply()
            .unwrap();
    }

    pub fn run(self) {
        self.set_logging();

        let addr = "0.0.0.0:8000";
        trace!("create router");
        let router = build_app_router(self.store_url);
        info!("server listening on 0.0.0.0:8000");

        start(addr, router);
    }
}
