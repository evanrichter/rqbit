use librqbit_core::id20::Id20;
use serde::Serializer;

pub fn serialize_id20<S>(id: &Id20, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ser.serialize_str(&id.as_string())
}
