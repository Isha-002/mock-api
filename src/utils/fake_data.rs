pub fn fake_data_sql() -> String {
    include_str!("../../migration/01_data.sql").to_string()
}
