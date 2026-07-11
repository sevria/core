use serde::Serialize;

use crate::Error;

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub struct ResponseBuilder<T: Serialize>(Response<T>);

impl<T: Serialize> ResponseBuilder<T> {
    pub fn new() -> Self {
        Self(Response {
            success: true,
            data: None,
            error: None,
        })
    }

    pub fn success(mut self, success: bool) -> Self {
        self.0.success = success;
        self
    }

    pub fn data(mut self, data: T) -> Self {
        self.0.data = Some(data);
        self
    }

    pub fn error(mut self, error: Error) -> Self {
        self.0.success = false;
        self.0.error = Some(error.to_string());
        self
    }

    pub fn build(self) -> Response<T> {
        self.0
    }
}
