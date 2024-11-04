// tests.rs
use rusqlite::{Connection, Result};
use my_crate::{create_table, drop_table, load_data_from_csv, query_exec}; // Use your actual crate name

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer scope (mainly to use rusqlite and the functions)

    // Helper function to create an in-memory SQLite database for testing
    fn setup_test_db() -> Result<Connection> {
        let conn = Connection::open_in_memory()?;
        create_table(&conn, "TestTable")?;
        Ok(conn)
    }

    #[test]
    fn test_create_table() {
        let conn = setup_test_db().expect("Failed to set up test database");
        let result = create_table(&conn, "AnotherTestTable");
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_data_from_csv() {
        let conn = setup_test_db().expect("Failed to set up test database");
        
        let csv_data = "competitorname,chocolate,fruity,caramel,peanutyalmondy,nougat,crispedricewafer,hard,bar,pluribus\n\
                        TestCandy,1,1,1,1,1,1,1,1,1\n";
        let file_path = "test_data.csv";
        std::fs::write(file_path, csv_data).expect("Failed to write test CSV");

        let result = load_data_from_csv(&conn, "TestTable", file_path);
        assert!(result.is_ok());

        std::fs::remove_file(file_path).expect("Failed to remove test CSV");
    }

    #[test]
    fn test_query_exec() {
        let conn = setup_test_db().expect("Failed to set up test database");
        
        let csv_data = "competitorname,chocolate,fruity,caramel,peanutyalmondy,nougat,crispedricewafer,hard,bar,pluribus\n\
                        TestCandy,1,1,1,1,1,1,1,1,1\n";
        let file_path = "test_data.csv";
        std::fs::write(file_path, csv_data).expect("Failed to write test CSV");
        load_data_from_csv(&conn, "TestTable", file_path).expect("Failed to load data");

        let query = "SELECT * FROM TestTable WHERE competitorname = 'TestCandy'";
        let result = query_exec(&conn, query);
        assert!(result.is_ok());

        std::fs::remove_file(file_path).expect("Failed to remove test CSV");
    }

    #[test]
    fn 
