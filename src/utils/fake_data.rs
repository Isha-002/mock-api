pub fn fake_data_sql() -> String {
    include_str!("../../migrations/01_data.sql").to_string()
}
