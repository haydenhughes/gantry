use crate::schema::exerciseentries;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;
use chrono::NaiveDateTime;

type AllColumns = (
    exerciseentries::exerciseentryid,
    exerciseentries::exercisetype,
    exerciseentries::endtime,
    exerciseentries::comments,
);

pub type All = Select<exerciseentries::table, AllColumns>;

pub type WithID<'a> = Eq<exerciseentries::exerciseentryid, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;

#[derive(Queryable, Serialize)]
pub struct ExerciseEntry {
    pub id: i32,
    pub exercisetype: i32,
    pub endtime: NaiveDateTime,
    pub comments: Option<String>
}

impl ExerciseEntry {
    pub fn with_id(id: &i32) -> WithID {
        exerciseentries::exerciseentryid.eq(id)
    }

    pub fn all() -> All {
        exerciseentries::table.select(exerciseentries::all_columns)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
