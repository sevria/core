use mola::MolaSchema;
use serde::Serialize;

// ---------------------------------------------------------------------------
// Success response — carries data and optional meta, no error
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct Response<D: Serialize, M: Serialize> {
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<D>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<M>,
}

impl<D: Serialize + MolaSchema, M: Serialize + MolaSchema> MolaSchema for Response<D, M> {
    fn description() -> &'static str {
        "Success response"
    }

    fn json_schema() -> serde_json::Value {
        let mut props = serde_json::Map::new();
        props.insert("success".into(), serde_json::json!({"type": "boolean"}));
        props.insert("data".into(), D::json_schema());
        props.insert("meta".into(), M::json_schema());
        serde_json::json!({
            "type": "object",
            "properties": props,
            "required": ["success"]
        })
    }
}

// ---------------------------------------------------------------------------
// Error response — carries error and optional meta, no data
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct ErrorResponse<M: Serialize> {
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<M>,
}

impl<M: Serialize + MolaSchema> MolaSchema for ErrorResponse<M> {
    fn description() -> &'static str {
        "Error response"
    }

    fn json_schema() -> serde_json::Value {
        let mut props = serde_json::Map::new();
        props.insert("success".into(), serde_json::json!({"type": "boolean"}));
        props.insert("error".into(), Error::json_schema());
        props.insert("meta".into(), M::json_schema());
        serde_json::json!({
            "type": "object",
            "properties": props,
            "required": ["success"]
        })
    }
}

// ---------------------------------------------------------------------------
// Error payload
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct Error {
    pub code: String,
    pub message: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}

impl MolaSchema for Error {
    fn description() -> &'static str {
        "Error details"
    }

    fn json_schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "code": {"type": "string"},
                "message": {"type": "string"},
                "details": {
                    "type": "array",
                    "items": ErrorDetail::json_schema()
                }
            },
            "required": ["code", "message"]
        })
    }
}

// ---------------------------------------------------------------------------
// Error detail
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct ErrorDetail {
    pub field: String,
    pub reason: String,
}

impl MolaSchema for ErrorDetail {
    fn description() -> &'static str {
        "Error detail"
    }

    fn json_schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "field": {"type": "string"},
                "reason": {"type": "string"}
            },
            "required": ["field", "reason"]
        })
    }
}
