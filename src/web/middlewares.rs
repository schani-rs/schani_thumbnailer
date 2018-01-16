use std::io;

use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::State;

use service::ThumbnailService;

pub struct ThumbnailServiceMiddleware {
    service: ThumbnailService,
}

impl ThumbnailServiceMiddleware {
    pub fn new(service: ThumbnailService) -> Self {
        ThumbnailServiceMiddleware { service: service }
    }
}

impl Middleware for ThumbnailServiceMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture> + 'static,
        Self: Sized,
    {
        state.put(ThumbnailServiceMiddlewareData::new(self.service.clone()));

        chain(state)
    }
}

impl NewMiddleware for ThumbnailServiceMiddleware {
    type Instance = ThumbnailServiceMiddleware;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(ThumbnailServiceMiddleware::new(self.service.clone()))
    }
}

#[derive(StateData)]
pub struct ThumbnailServiceMiddlewareData {
    service: ThumbnailService,
}

impl ThumbnailServiceMiddlewareData {
    pub fn new(service: ThumbnailService) -> Self {
        ThumbnailServiceMiddlewareData { service: service }
    }

    pub fn service(&self) -> &ThumbnailService {
        &self.service
    }
}
