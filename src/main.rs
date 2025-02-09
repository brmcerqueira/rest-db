use heed::{Database, EnvOpenOptions};
use heed::types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = unsafe { EnvOpenOptions::new().open("db")? };

    // We open the default unnamed database
    let mut wtxn = env.write_txn()?;
    let db: Database<Str, Str> = env.create_database(&mut wtxn, None)?;

    // We open a write transaction
    db.put(&mut wtxn, "seven", "seven2")?;
    db.put(&mut wtxn, "zero", "zero2")?;
    db.put(&mut wtxn, "five", "five2")?;
    db.put(&mut wtxn, "three", "three2")?;

    wtxn.commit()?;

    wtxn = env.write_txn()?;

    let mut iter = db.iter_mut(&mut wtxn)?;

    while let Some(item) = iter.next() {
        println!("{}", item.unwrap().1);
    }

    Ok(())
}