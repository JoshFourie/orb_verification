use crate::schema::true_orbs;
use serde_derive::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct TrueOrb {
    pub id: i32,
    pub proof: String,
    pub verified_values: String,
    pub signature: String,
    pub public_key: String,
    pub common_reference_string: String,
}

#[derive(Insertable)]
#[table_name="true_orbs"]
pub struct InsertOrb {
    pub proof: String,
    pub verified_values: String,
    pub signature: String,
    pub public_key: String,
    pub common_reference_string: String,
}

impl InsertOrb {
    pub fn into(
        proof: String,
        ver: String,
        sig: String,
        pub_key: String,
        crs: String,
    ) -> Self {
        InsertOrb {
            proof: proof,
            verified_values: ver,
            signature: sig,
            public_key: pub_key,
            common_reference_string: crs,
        }
    }
    pub fn into_from_tuple(tuple: (String, String, String, String, String)) -> Self {
        Self::into(
            tuple.0,
            tuple.1,
            tuple.2,
            tuple.3,
            tuple.4,
        )
    }
}