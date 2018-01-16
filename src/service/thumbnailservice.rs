use futures::Future;
use hyper::Uri;
use image::{load_from_memory, FilterType, GenericImage, ImageFormat};
use tokio_core::reactor::Handle;

use schani_store_client::StoreClient;

#[derive(Clone)]
pub struct ThumbnailService {
    store_uri: Uri,
}

impl ThumbnailService {
    pub fn new(store_uri: Uri) -> Self {
        ThumbnailService {
            store_uri: store_uri,
        }
    }

    pub fn get_thumbnail(
        &self,
        id: &String,
        handle: &Handle,
    ) -> Box<Future<Item = Vec<u8>, Error = ()>> {
        let store_client = StoreClient::new(self.store_uri.clone(), handle);

        let f = store_client.get_image(id).and_then(|image| {
            let img = load_from_memory(image.as_slice()).expect("could not load image");
            let dims = img.dimensions();

            // Assuming the image lib creates the max possible rescaled image while
            // preserving the aspect ratio, we only have to define the shorter side
            // of the image
            let new_dim = if dims.0 == dims.1 {
                (250, 250)
            } else if dims.0 < dims.1 {
                // Portrait
                (250, dims.1)
            } else {
                // Landscape
                (dims.0, 250)
            };
            info!(
                "original image is {}x{}, cropping to {}x{}",
                dims.0, dims.1, new_dim.0, new_dim.1
            );
            let mut resized = img.resize(new_dim.0, new_dim.1, FilterType::Triangle);
            let resized_dims = resized.dimensions();
            info!("resized image is {}x{}", resized_dims.0, resized_dims.1);

            let offset_x = (resized_dims.0 - 250) / 2;
            let offset_y = (resized_dims.1 - 250) / 2;
            let cropped = resized.crop(offset_x as u32, offset_y as u32, 250, 250);

            let mut res = vec![];
            cropped
                .save(&mut res, ImageFormat::JPEG)
                .expect("could not write image");

            Ok(res)
        });

        Box::new(f.map_err((|_| ())))
    }
}
