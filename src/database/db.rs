// sled db to store and fetch scenarios
use crate::database::decode::decompress_string;
use crate::database::types::Urldata;
use crate::error::Error as CrateError;
use crate::scenarios::scenario_parse::generate_random_id;
use anyhow::Error;
use sled;
use sled::{Db, IVec}; //IVec Tree

use std::str;

#[derive(Debug, Clone)]
pub struct DBhandler {}

/// sled db handler for bagpipes
impl DBhandler {
    /// return a sled::Db instance
    pub fn read_db(&self) -> Result<Db, Error> {
        let open: Db = sled::open("bp.db")?;
        return Ok(open);
    }
    /*
        /// decode the ts encoded blob
        pub async fn decode_entry(&self, input: String) -> Result<String, Error> {
            let outp = decompress_string(input).await?;
            return Ok(outp);
        }
    */
    /// save entry in database
    pub fn saveurl(&self, longurl: Urldata) -> Result<String, Error> {
        let url_data = IVec::from(longurl.url.as_bytes());
        let my_id = generate_random_id();

        let db_instance: Db = self.read_db()?;
        db_instance.insert(my_id.clone(), url_data)?;
        db_instance.flush()?;
        Ok(my_id)
    }
    /// return entry in the db
    pub fn get_entry(&self, key: String) -> Result<String, CrateError> {
        let db: Db = self.read_db()?; //  lots of io usage
        match db.get(key.as_bytes()) {
            Ok(Some(value)) => {
                let outputen: String = String::from_utf8(value.to_vec()).expect("Invalid UTF-8");
                return Ok(outputen);
            }
            _ => return Err(CrateError::NoEntryInDb),
        }
    }
}
