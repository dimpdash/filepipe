use std::sync::Arc;

use serde::{de, Deserialize, Deserializer, Serializer};

use crate::{app::APP, domain::program::Program};

pub fn serialize_id<S>(rc: &Arc<dyn Program>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value = rc.get_name();
    serializer.serialize_str(value.as_str())
}

pub fn deserialize_id<'de, D>(deserializer: D) -> Result<Arc<dyn Program>, D::Error>
where
    D: Deserializer<'de>,
{
    let id = String::deserialize(deserializer)?;

    let mut app = APP.lock().unwrap();
    let mut program_repo = app.get_program_repo();

    program_repo
        .find(&id)
        .map_err(|_| de::Error::custom("key not found"))
}
