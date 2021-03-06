///! Implementation of a container for basic asset data
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::currency::Currency;
use super::{DataError, DataItem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub id: Option<usize>,
    pub asset: usize,
    pub name: String,
    pub currency: Currency,
    pub source: String,
    pub priority: i32,
    pub factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: Option<usize>,
    pub ticker: usize,
    pub price: f64,
    pub time: DateTime<Utc>,
    pub volume: Option<f64>,
}

impl DataItem for Quote {
    // get id or return error if id hasn't been set yet
    fn get_id(&self) -> Result<usize, DataError> {
        match self.id {
            Some(id) => Ok(id),
            None => Err(DataError::DataAccessFailure(
                "tried to get id of temporary quote".to_string(),
            )),
        }
    }
    // set id or return error if id has already been set
    fn set_id(&mut self, id: usize) -> Result<(), DataError> {
        match self.id {
            Some(_) => Err(DataError::DataAccessFailure(
                "tried to change valid quote id".to_string(),
            )),
            None => {
                self.id = Some(id);
                Ok(())
            }
        }
    }
}

impl DataItem for Ticker {
    // get id or return error if id hasn't been set yet
    fn get_id(&self) -> Result<usize, DataError> {
        match self.id {
            Some(id) => Ok(id),
            None => Err(DataError::DataAccessFailure(
                "tried to get id of temporary ticker".to_string(),
            )),
        }
    }
    // set id or return error if id has already been set
    fn set_id(&mut self, id: usize) -> Result<(), DataError> {
        match self.id {
            Some(_) => Err(DataError::DataAccessFailure(
                "tried to change valid ticker id".to_string(),
            )),
            None => {
                self.id = Some(id);
                Ok(())
            }
        }
    }
}
