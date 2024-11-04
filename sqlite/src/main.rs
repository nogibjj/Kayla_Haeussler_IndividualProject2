use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result}; // Only rusqlite is needed if that's the DB library
use sqlite::create_table; // Adjust the import to match your module structure
use sqlite::drop_table; // Adjust import for drop_table
use sqlite::load_data_from_csv; // Adjust import for load_data_from_csv
use sqlite::query_exec; // Adjust import for query_exec

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(alias = "c", short_flag = 'c')]
    Create { table_name: String },
    #[command(alias = "q", short_flag = 'q')]
    Query { query: String },
    #[command(alias = "d", short_flag = 'd')]
    Delete { delete_query: String },
    #[command(alias = "l", short_flag = 'l')]
    Load {
        table_name: String,
        file_path: String,
    },
    #[command(alias = "u", short_flag = 'u')]
    Update {
        table_name: String,
        set_clause: String,
        condition: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let conn = Connection::open("my_database.db")?;

    match args.command {
        Commands::Create { table_name } => {
            println!("Creating Table {}", table_name);
            create_table(&conn, &table_name).expect("Failed to create table");
        }
        Commands::Query { query } => {
            println!("Query: {}", query);
            query_exec(&conn, &query).expect("Failed to execute query");
        }
        Commands::Delete { delete_query } => {
            println!("Deleting: {}", delete_query);
            drop_table(&conn, &delete_query).expect("Failed to drop table");
        }
        Commands::Load {
            table_name,
            file_path,
        } => {
            println!(
                "Loading data into table '{}' from '{}'",
                table_name, file_path
            );
            load_data_from_csv(&conn, &table_name, &file_path)
                .expect("Failed to load data from csv");
        }
        Commands::Update {
            table_name,
            set_clause,
            condition,
        } => {
            let query = format!(
                "UPDATE {} SET {} WHERE {};",
                table_name, set_clause, condition
            );
            println!("Executing update: {}", query);
            query_exec(&conn, &query).expect("Failed to execute update");
        }
    }
    Ok(())
}
