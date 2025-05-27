

pub fn init_sql() -> String {
    include_str!("../../initial_schema/0v1_schema.sql").to_string()
}

pub fn reset_sql() -> String {
    include_str!("../../initial_schema/0v1_reset.sql").to_string()
}

pub fn sample_data_sql() -> String {
    include_str!("../../initial_schema/0v1_sample.sql").to_string()
}