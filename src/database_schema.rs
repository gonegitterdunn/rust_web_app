// table! macro = table exists in the db
table! {
  to_do (id) {
    id -> Int4,
    title -> Varchar,
    status -> Varchar,
  }
}
