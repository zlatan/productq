use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Parameters {
    pub q: String,
}
