use crate::database_schema::to_do;

#[derive(Insertable)]
#[table_name = "to_do"]
pub struct NewItem {
    pub title: String,
    pub status: String,
}

impl NewItem {
    pub fn new(title: String) -> Self {
        Self {
            title,
            status: "pending".to_string(),
        }
    }
}
