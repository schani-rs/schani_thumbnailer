use gotham::router::Router;
use gotham::router::builder::*;
use gotham::router::route::dispatch::{finalize_pipeline_set, new_pipeline_set};
use gotham::pipeline::new_pipeline;

use super::extractors::ImageRequestPath;
use super::handlers::ThumbnailController;
use super::middlewares::ThumbnailServiceMiddleware;
use service::ThumbnailService;

pub fn build_app_router(store_uri: &str) -> Router {
    trace!("build pipelines");
    let pipelines = new_pipeline_set();
    let (pipelines, default) = pipelines.add(
        new_pipeline()
            .add(ThumbnailServiceMiddleware::new(ThumbnailService::new(
                store_uri.parse().unwrap(),
            )))
            .build(),
    );
    let pipelines = finalize_pipeline_set(pipelines);
    let default_pipeline_chain = (default, ());

    // Router builder starts here
    trace!("finalize router");
    build_router(default_pipeline_chain, pipelines, |route| {
        route
            .get("/thumb/:id")
            .with_path_extractor::<ImageRequestPath>()
            .to(ThumbnailController::get_thumbnail);
    })
}
