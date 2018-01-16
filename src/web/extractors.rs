#[derive(StateData, PathExtractor, StaticResponseExtender)]
pub struct ImageRequestPath {
    id: String,
}

impl ImageRequestPath {
    pub fn id(&self) -> String {
        self.id.to_owned()
    }
}
