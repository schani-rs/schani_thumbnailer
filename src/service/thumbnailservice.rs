use futures::Future;
use hyper::Uri;
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
        id: &   String,
        handle: &Handle,
    ) -> Box<Future<Item = Vec<u8>, Error = ()>> {
        let store_client = StoreClient::new(self.store_uri.clone(), handle);

        Box::new(store_client.get_image(id).map_err((|_| ())))
    }

    /*
    pub fn add_raw_file(
        &self,
        handle: &Handle,
        data: Vec<u8>,
    ) -> Box<Future<Item = String, Error = ()>> {
        info!("got {} bytes raw image", data.len());

        let store_client = StoreClient::new(self.store_uri.clone(), handle);

        Box::new(store_client.upload_raw_image(data).map_err(|_| ()))
    }

    pub fn add_sidecar(
        &self,
        handle: &Handle,
        data: Vec<u8>,
    ) -> Box<Future<Item = String, Error = ()>> {
        info!("got {} bytes sidecar", data.len());

        let store_client = StoreClient::new(self.store_uri.clone(), handle);

        Box::new(store_client.upload_sidecar(data).map_err(|_| ()))
    }

    pub fn add_image(
        &self,
        handle: &Handle,
        data: Vec<u8>,
    ) -> Box<Future<Item = String, Error = ()>> {
        info!("got {} bytes image", data.len());

        let store_client = StoreClient::new(self.store_uri.clone(), handle);

        Box::new(store_client.upload_image(data).map_err(|_| ()))
    }

    pub fn finish_import(
        &self,
        conn: &PgConnection,
        import_id: i32,
        handle: &Handle,
    ) -> Box<Future<Item = Import, Error = ()>> {
        let import = self.delete_import(conn, import_id);

        let lib_client = LibraryClient::new(self.library_uri.clone(), handle);

        let data = NewImageData {
            raw_id: import.raw_image_id.to_owned(),
            sidecar_id: import.sidecar_id.to_owned(),
            image_id: import.image_id.to_owned(),
            user_id: import.user_id.to_owned(),
        };

        let f = lib_client
            .add_image(data)
            .and_then(move |id| {
                info!("image {} imported successfully", id);
                send_processing_message(id);
                info!("image id {} pushed to processing queue", id);
                Ok(import)
            })
            .map_err(|_| ());

        Box::new(f)
    }
    */
}
