use bigdecimal::{BigDecimal, ToPrimitive};
use serde::ser::Serializer;

pub fn serialize_bigdecimal_opt<S>(bg: &Option<BigDecimal>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match bg {
     Some(b) => serializer.serialize_f64(b.to_f64().unwrap()),
     None => serializer.serialize_none(),
    }
}

pub fn serialize_bigdecimal<S>(bg: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_f64(bg.to_f64().unwrap())
}