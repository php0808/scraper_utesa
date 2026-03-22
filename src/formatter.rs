use serde_json::json;

use crate::entities::{Response};

pub fn to_json(doc: Response) ->serde_json::Value{
   json!(doc)
}