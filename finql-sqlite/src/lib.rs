///! Implementation of sqlite3 data handler

use rusqlite::{Connection,NO_PARAMS};
use chrono::{DateTime, Utc};

use finql_data::{CurrencyConverter, Currency, CurrencyError, QuoteHandler};

mod raw_transaction;
pub mod asset_handler;
pub mod quote_handler;
pub mod transaction_handler;

/// Struct to handle connections to sqlite3 databases
pub struct SqliteDB<'a> {
    /// conn is made public to allow extending this struct outside of the library
    pub conn: &'a Connection,
}

impl<'a> SqliteDB<'_> {

    /// Initialize new database by creating table, fill
    pub fn init(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS assets (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                wkn TEXT UNIQUE,
                isin TEXT UNIQUE,
                note TEXT
            )",
            NO_PARAMS,
        )?;
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY,
                trans_type TEXT NOT NULL,
                asset_id INTEGER,
                cash_amount REAL NOT NULL,
                cash_currency TXT NOT NULL,
                cash_date TEXT NOT NULL,
                related_trans KEY,
                position REAL,
                note TEXT,
                FOREIGN KEY(asset_id) REFERENCES assets(id),
                FOREIGN KEY(related_trans) REFERENCES transactions(id)
            );",
            NO_PARAMS,
        )?;
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS ticker (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                asset_id INTEGER NOT NULL,
                source TEXT NOT NULL,
                priority INTEGER NOT NULL,
                currency TEXT NOT NULL,
                factor REAL NOT NULL DEFAULT 1.0,
                FOREIGN KEY(asset_id) REFERENCES assets(id) 
            );",
            NO_PARAMS,
        )?;
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS quotes (
                id INTEGER PRIMARY KEY,
                ticker_id INTEGER NOT NULL,
                price REAL NOT NULL,
                time TEXT NOT NULL,
                volume REAL,
                FOREIGN KEY(ticker_id) REFERENCES ticker(id) );",
            NO_PARAMS,
        )?;
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS rounding_digits (
                id INTEGER PRIMARY KEY,
                currency TEXT NOT NULL UNIQUE,
                digits INTEGER NOT NULL);",
            NO_PARAMS,
        )?;
        Ok(())
    }
}

impl CurrencyConverter for SqliteDB<'_> {
    fn fx_rate(&mut self, foreign_currency: Currency, domestic_currency: Currency, time: DateTime<Utc>) -> Result<f64, CurrencyError> {
        if foreign_currency == domestic_currency {
            return Ok(1.0);
        }

        let (fx_quote, quote_currency) =
            self.get_last_quote_before(&foreign_currency.to_string(), time)
                .map_err(|_| CurrencyError::ConversionFailed)?;
        if quote_currency == domestic_currency {
            return Ok(fx_quote.price);
        }
        
        let (fx_quote, quote_currency) =
        self.get_last_quote_before(&domestic_currency.to_string(), time)
            .map_err(|_| CurrencyError::ConversionFailed)?;
        if quote_currency == foreign_currency {
            return Ok(1./fx_quote.price);
        }
        Err(CurrencyError::ConversionFailed)
    }
}