
///为表进行编码命名
pub fn code_table_name(table_name: &str, db_name: &str, ) -> String {
    let mut name = String::new();
    name.push_str("t_");
    name.push_str(&table_name);
    name.push_str(&db_name);
    name
}

