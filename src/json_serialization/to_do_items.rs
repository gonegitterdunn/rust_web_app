use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoItems {
    pub done_items: Vec<Base>,
    pub pending_items: Vec<Base>,
    pub done_items_count: i8,
    pub pending_items_count: i8,
}

impl Responder for ToDoItems {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut pending_items = Vec::new();
        let mut done_items = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Done(packed) => done_items.push(packed.base_struct),
                ItemTypes::Pending(packed) => pending_items.push(packed.base_struct),
            }
        }

        let done_items_count: i8 = done_items.len() as i8;
        let pending_items_count: i8 = pending_items.len() as i8;

        Self {
            done_items,
            pending_items,
            done_items_count,
            pending_items_count,
        }
    }
}
