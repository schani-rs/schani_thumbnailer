use futures::Future;
use gotham::handler::HandlerFuture;
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use hyper::StatusCode;
use mime;
use tokio_core::reactor::Handle;

use super::extractors::ImageRequestPath;
use super::middlewares::ThumbnailServiceMiddlewareData;

pub struct ThumbnailController;

impl ThumbnailController {
    pub fn get_thumbnail(state: State) -> Box<HandlerFuture> {
        let get_image = {
            let id = ImageRequestPath::borrow_from(&state).id();
            let handle = Handle::borrow_from(&state);
            let service = state.borrow::<ThumbnailServiceMiddlewareData>().service();

            service.get_thumbnail(&id, handle).join(Ok(id))
        };

        let f = get_image
            .and_then(move |(image, id)| {
                info!("loaded image {} ({} bytes)", id, image.len());
                let resp = create_response(&state, StatusCode::Ok, Some((image, mime::IMAGE_JPEG)));
                Ok((state, resp))
            })
            .map_err(|_| unimplemented!());

        Box::new(f)
    }
}
