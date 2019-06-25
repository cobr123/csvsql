extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use rusqlite::vtab::csvtab;

#[derive(Debug)]
struct Data {
    title: String,
    body: String
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    csvtab::load_module(&conn).unwrap();

    //https://www.kaggle.com/stackoverflow/stacksample/downloads/stacksample.zip
    conn.execute(
        "create virtual table questions using csv(filename='questions.csv', header=yes)",
        NO_PARAMS,
    )?;
    conn.execute(
        "create virtual table answers using csv(filename='answers.csv', header=yes)",
        NO_PARAMS,
    )?;
    conn.execute(
        "create virtual table tags using csv(filename='tags.csv', header=yes)",
        NO_PARAMS,
    )?;
    let mut stmt = conn
        .prepare("select q.title ,a.body from questions q, answers a where q.id = a.ParentId limit 1")?;

    let data_iter = stmt
        .query_map(NO_PARAMS, |row| Ok(Data {
            title: row.get(0)?,
            body: row.get(1)?
        }))?;

    for data in data_iter {
        println!("Found person {:?}", data?);
    }

    Ok(())
}