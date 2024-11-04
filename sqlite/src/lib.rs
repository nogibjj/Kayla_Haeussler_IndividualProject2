use csv::ReaderBuilder; // for loading from csv
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File; // for loading csv

// Create a table
pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS \"{}\" (
            competitorname TEXT PRIMARY KEY,
            chocolate INTEGER NOT NULL,
            fruity INTEGER NOT NULL,
            caramel INTEGER NOT NULL,
            peanutyalmondy INTEGER NOT NULL,
            nougat INTEGER NOT NULL,
            crispedricewafer INTEGER NOT NULL,
            hard INTEGER NOT NULL,
            bar INTEGER NOT NULL,
            pluribus INTEGER NOT NULL
        )",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(())
}

// Read
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    let mut stmt = conn.prepare(query_string)?;

    // Use query_map to handle multiple rows
    let rows = stmt.query_map([], |row| {
        let competitorname: String = row.get(0)?;
        let chocolate: i32 = row.get(1)?;
        let fruity: i32 = row.get(2)?;
        let caramel: i32 = row.get(3)?;
        let peanutyalmondy: i32 = row.get(4)?;
        let nougat: i32 = row.get(5)?;
        let crispedricewafer: i32 = row.get(6)?;
        let hard: i32 = row.get(7)?;
        let bar: i32 = row.get(8)?;
        let pluribus: i32 = row.get(9)?;
        Ok((
            competitorname,
            chocolate,
            fruity,
            caramel,
            peanutyalmondy,
            nougat,
            crispedricewafer,
            hard,
            bar,
            pluribus,
        ))
    })?;

    // Iterate over the rows and print the results
    for row in rows {
        let (
            competitorname,
            chocolate,
            fruity,
            caramel,
            peanutyalmondy,
            nougat,
            crispedricewafer,
            hard,
            bar,
            pluribus,
        ) = row?;
        println!(
            "Competitor Name: {}, Chocolate: {}, Fruity: {}, Caramel: {}, Peanuty/Almondy: {}, Nougat: {}, Crisped Rice/Wafer: {}, Hard Candy: {}, Bar: {}, One of Many in a Bag/Box: {}",
            competitorname, chocolate, fruity, caramel, peanutyalmondy, nougat, crispedricewafer, hard, bar, pluribus
        );
    }

    Ok(())
}

// Delete
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

// Load data from a CSV file into a table
pub fn load_data_from_csv(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let insert_query = format!(
        "INSERT INTO \"{}\" (competitorname, chocolate, fruity, caramel, peanutyalmondy, nougat, crispedricewafer, hard, bar, pluribus) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        table_name
    );

    // Loop through the records in the CSV file
    for result in rdr.records() {
        let record = result?;
        let competitorname: &str = &record[0];
        let chocolate: i32 = record[1].parse()?;
        let fruity: i32 = record[2].parse()?;
        let caramel: i32 = record[3].parse()?;
        let peanutyalmondy: i32 = record[4].parse()?;
        let nougat: i32 = record[5].parse()?;
        let crispedricewafer: i32 = record[6].parse()?;
        let hard: i32 = record[7].parse()?;
        let bar: i32 = record[8].parse()?;
        let pluribus: i32 = record[9].parse()?;

        conn.execute(
            &insert_query,
            params![
                competitorname,
                chocolate,
                fruity,
                caramel,
                peanutyalmondy,
                nougat,
                crispedricewafer,
                hard,
                bar,
                pluribus
            ],
        )?;
    }

    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}
