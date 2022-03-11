use rocket::http::Status;
use rocket::serde::json::Json;

pub mod participant;
pub mod prize;
pub mod promo;

fn get_status<T: Default>(result: Result<T, ()>) -> (Status, Json<T>) {
    match result {
        Ok(value) => (Status::Ok, Json(value)),
        Err(_) => (Status::BadRequest, Json(T::default()))
    }
}
