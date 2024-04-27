// sled db to store and fetch scenarios
use crate::database::decode::decompress_string;
use crate::database::types::{TxInfo, Urldata};
use crate::error::Error as CrateError;
use crate::scenarios::scenario_parse::generate_random_id;
use crate::scenarios::scenario_types::Graph;
use anyhow::Error;
use bincode::{deserialize, serialize};
use chrono::Utc;
use sled;
use sled::{Db, IVec}; //IVec Tree
use std::str;

use polodb_core::Database as PoloDB;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub msg: String,
    pub Date: String,
}

#[derive(Debug, Clone)]
pub struct DBhandler {}

fn custom_merge_operator() -> impl Fn(&[u8], Option<&[u8]>, &[u8]) -> Option<Vec<u8>> {
    |_, existing_value, merged_bytes| {
        let mut merged = existing_value.map_or_else(Vec::new, |iv| iv.to_vec());
        merged.extend_from_slice(merged_bytes);
        Some(merged)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Loghandler {}

/// Polodb
impl Loghandler {
    /// read the logs.db
    pub fn read_db(&self) -> Result<PoloDB, Error> {
        let open: PoloDB = PoloDB::open_file("logs.db")?;
        return Ok(open);
    }
    /// insert transaction to the Mempool
    pub fn insert_tx(
        &self,
        scenario_id: String,
        amount: String,
        chain: String,
        txType: String,
    ) -> Result<(), Error> {
        let db = self.read_db()?;
        let tx_pool = format!("{}_txpool", scenario_id);
        let collection = db.collection::<TxInfo>(tx_pool.as_str());
        collection.insert_one(TxInfo {
            amount: amount,
            chain: chain,
            txType: txType,
            Date: "".to_string(),
        })?;
        Ok(())
    }

    /// get mempool for scenarioid
    pub fn get_transactions(&self, scenario_id: String) -> Result<Vec<TxInfo>, Error> {
        let db = self.read_db()?;
        let tx_pool = format!("{}_txpool", scenario_id);

        let collection = db.collection::<TxInfo>(tx_pool.as_str());
        let entries = collection.find(None)?; // return all entries under parent key
        let listan: Vec<TxInfo> = entries.into_iter().map(|entry| entry.unwrap()).collect();
        Ok(listan)
    }

    /// insert a log into the log files
    pub fn insert_logs(&self, scenario_id: String, message: String) -> Result<(), Error> {
        let db = self.read_db()?;
        let collection = db.collection::<LogEntry>(scenario_id.as_str());
        collection.insert_one(LogEntry {
            msg: message,
            Date: "now".to_string(),
        })?;
        Ok(())
    }

    /// Returns all logs in the Vec<LogEntry> format
    pub fn get_entry(&self, scenario_id: String) -> Result<Vec<LogEntry>, Error> {
        let db = self.read_db()?;
        let collection = db.collection::<LogEntry>(scenario_id.as_str());
        let entries = collection.find(None)?; // return all entries under parent key
        let listan: Vec<LogEntry> = entries.into_iter().map(|entry| entry.unwrap()).collect();
        Ok(listan)
    }

    /// return all entries in the log
    pub fn get_all_entries(&self) -> Result<Vec<LogEntry>, Error> {
        let db = self.read_db()?;
        let collection = db.collection::<LogEntry>("logs");
        let entries = collection.find(None)?;
        let listan: Vec<LogEntry> = entries.into_iter().map(|entry| entry.unwrap()).collect();

        Ok(listan)
    }
    pub fn new() -> Loghandler {
        return Loghandler {};
    }
}

/// sled db handler for bagpipes
impl DBhandler {
    /// return a sled::Db instance
    pub fn read_db(&self) -> Result<Db, Error> {
        let open: Db = sled::open("bp.db")?;
        // lets define our merging operations
        let merge_result = open.set_merge_operator(custom_merge_operator());
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
    /// println! db stats
    pub fn display_stats(&self) -> Result<(), CrateError> {
        let db = self.read_db()?;
        let amount_of_entries = count_entries(&db);
        let size = db.size_on_disk()?;
        println!("[Database Checker] - Metadata stats:");
        println!(
            "[Database Checker] - Amount of entries in the database: {:?}",
            amount_of_entries
        );
        println!("[Database Checker] - Size on disk: {:?}", size);

        Ok(())
    }
    /// query for an item and decode it to a Graph
    pub async fn get_decoded_entry(&self, key: String) -> Result<Graph, CrateError> {
        let out = self.get_entry(key)?;
        let decoded = decompress_string(out)
            .await
            .expect("Failed to decompress string, invalid value");

        let graph: Graph = serde_json::from_str(decoded.as_str()).expect("Failed to parse JSON");
        return Ok(graph);
    }
    /*     FUNCTIONS MOVED TO LOGHANDLER
        /// save the logs of a thread to a db tree / list
        pub fn save_log(&self, thread_name: String, log_entry: String) -> Result<bool, CrateError> {
            let db = self.read_db()?;
            let utc_time = Utc::now().to_string(); // Convert UTC time to string
            let formated_date = &utc_time[..19]; // Slice the first 19 characters
            let date_n_log = format!("{} {}NEXTENTRY222", formated_date, log_entry);
            let log_name_entry = format!("{}_logs", thread_name);
            let op_bytes = serialize(&date_n_log).unwrap();
            db.merge(log_name_entry.as_bytes(), op_bytes).unwrap();
            // Serialize the values and insert them into the database

         //     let tree = db.open_tree(thread_name.clone()).expect("Failed to open tree");
         //     tree.merge(thread_name, date_n_log)?;
          //    db.flush()?;3

            Ok(true) // save log to
        }

        /// query the logs of a scenario worker
        pub fn query_logs(&self, thread_name: String) -> Result<Vec<String>, CrateError> {
            println!("query logs called");
            let db = self.read_db()?;

            let log_name_entry = format!("{}_logs", thread_name); // either we get a new db for the logs or just add a prefix
            println!("log name: {:?}", log_name_entry);

       /*     let outme = String::from_utf8(
                db.get(log_name_entry.clone())?
                    .expect("Could not get logs")
                    .to_vec(),
            )?;
    */
            let outme = match db.get(log_name_entry.as_bytes()) {
                Ok(Some(value)) => {
                    let val = deserialize(&value).expect("Could not deserialized");

                    let outputen: String = String::from_utf8(val).expect("Invalid UTF-8");
                    outputen
                }
                _ => "not found".to_string(), //return Err(CrateError::NoEntryInDb)
            };
            // remove null \0 chars in string

            let fmt_me = format!("{}", outme);
            let cleaned_str: String = fmt_me.chars().filter(|&c| c != '\0').collect();

                println!("Raw logs: {:?}", outme);
                println!("Raw logs 2: {}", fmt_me);
                println!("Cleaned string: {}", cleaned_str);
                println!("------------------------eol------------------------");
            /*
                    // new line is: %\0\0\0\0\0\0\0 and )\0\0\0\0\0\0\0"
                    let logs: Vec<String> = outme
                        .split_terminator("%\0\0\0\0\0\0\0")
                        .flat_map(|s| s.split_terminator(")\0\0\0\0\0\0\0"))
                        .map(|s| s.to_string())
                        .collect();
               let logs: Vec<String> = fmt_me
            .split_terminator(r"NEXTENTRY2222")
            .flat_map(|s| s.split_terminator(r"NEXTENTRY2222"))
            .map(|s| s.to_string())
            .collect();
            */


         let entries_test: Vec<String> = cleaned_str.split("NEXTENTRY222").map(|s| s.to_string()).collect();
            println!("entries test is: {:?}", entries_test);
        //      println!("Got logs: {:?}", logs);
            //      println!("filtered list: {:?}", logs);
            println!("query logs eol");
            return Ok(entries_test);
        }
    */
    pub fn new() -> DBhandler {
        return DBhandler {};
    }
}

fn count_entries(db: &Db) -> usize {
    let mut total_entries = 0;

    // Iterate over all entries and count them
    for _ in db.iter() {
        total_entries += 1;
    }

    total_entries
}
