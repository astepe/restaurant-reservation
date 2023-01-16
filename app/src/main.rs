use crate::routes::{index, post_reservation};
	
#[cfg(test)] mod tests;
mod repository;
mod services;

#[macro_use]
extern crate rocket;
mod models;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, post_reservation])
}
