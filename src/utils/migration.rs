pub fn migration_sql() -> String {
    include_str!("../../migrations/01_tables.sql").to_string()
}
