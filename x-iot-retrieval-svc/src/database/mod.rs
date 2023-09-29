use diesel_derive_enum::DbEnum;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::pg::PgConnection;
use serde::Deserialize;

pub type PgPooled = PooledConnection<ConnectionManager<PgConnection>>;

pub mod schema;

// tag::db_enum_custom[]
#[derive(DbEnum, Debug, Eq, PartialEq, Deserialize, Clone)] //<1>
#[DieselType = "Media_Enum_Map"] //<2>
pub enum MediaEnum { //<3>
    Image,
    Video,
    Unknown,
}
// end::db_enum_custom[]

#[derive(DbEnum, Debug, Eq, PartialEq, Deserialize, Clone)]
#[DieselType = "Location_Enum_Map"]
// tag::enum[]
pub enum LocationEnum {
    S3,
    Local
}
// end::enum[]

// tag::db_enum[]
#[derive(DbEnum, Debug, Eq, PartialEq, Deserialize, Clone)]
#[DieselType = "Media_Audience_Enum_Map"]
pub enum MediaAudienceEnum {
    Personal,
    Friends,
    Family
}
// end::db_enum[]
