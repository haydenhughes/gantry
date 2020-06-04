#![feature(proc_macro_hygiene, decl_macro, bool_to_option)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

pub mod models;
pub mod routes;
pub mod schema;

// So other applications (nudge, nudge; wink, wink) can hook into fodmap's auth
pub use models::User;

#[database("fodmap_db")]
pub struct FodmapDbConn(diesel::PgConnection);