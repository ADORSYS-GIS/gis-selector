use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Applicant {
    pub id: i32,
    pub name: String,
    pub experience: i32,
    pub education: String,
    pub location: String,
}
