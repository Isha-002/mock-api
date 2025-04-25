pub fn migration_sql() -> String {
    include_str!("../../migration/01_tables.sql").to_string()
}
