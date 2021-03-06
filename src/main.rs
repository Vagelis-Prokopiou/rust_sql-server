// apt-get install unixodbc-dev
// https://docs.microsoft.com/en-us/sql/connect/odbc/linux-mac/installing-the-microsoft-odbc-driver-for-sql-server?view=sql-server-ver15#debian17

extern crate odbc;
extern crate env_logger;

use odbc::*;
use std::io;
use odbc::ResultSetState::{NoData, Data};

fn execute_statement<T: odbc::odbc_safe::AutocommitMode>(conn: &Connection<T>) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;

    let mut sql_text = String::new();
    println!("Please enter SQL statement string: ");
    io::stdin().read_line(&mut sql_text).unwrap();

    match stmt.exec_direct(&sql_text)? {
        Data(mut stmt) => {
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                for i in 1..(cols + 1) {
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => print!(" {}", val),
                        None => print!(" NULL"),
                    }
                }
                println!("");
            }
        }
        NoData(_) => println!("Query executed, no data returned"),
    }

    Ok(())
}

fn connect() -> std::result::Result<(), DiagnosticRecord> {
    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    let mut buffer = String::new();
    // println!("Please enter connection string: ");
    // io::stdin().read_line(&mut buffer).unwrap();
    buffer = "Driver={ODBC Driver 17 for SQL Server}; Server=localhost; Database=test; UID=SA; PWD=123456aA!;".parse().unwrap();

    let conn = env.connect_with_connection_string(&buffer)?;
    execute_statement(&conn)
}

fn main() {
    env_logger::init();
    match connect() {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {}", diag),
    }
}



