use std::cell::RefCell;
use std::collections::HashMap;

use crate::models::Reservation;
use crate::repository::Repository;
use crate::rocket;
use crate::services::ReservationService;
use chrono::NaiveDateTime;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_returns_json() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.content_type(),
        Some(rocket::http::ContentType::JSON)
    );
}

#[test]
fn test_posts_valid_reservation() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    let response = client
        .post("/reservations")
        .header(rocket::http::ContentType::JSON)
        .json(&Reservation {
            id: 1,
            name: "Katinka Ingabogovinanana".to_string(),
            email: "katinka@example.com".to_string(),
            date: NaiveDateTime::parse_from_str(
                "2019-08-15T17:41:18.106108",
                "%Y-%m-%dT%H:%M:%S.%f",
            )
            .unwrap(),
            quantity: 1,
        })
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.content_type(),
        Some(rocket::http::ContentType::JSON)
    );
}

struct FakeRepository {
    data: HashMap<i32, Reservation>,
}

impl FakeRepository {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Repository<Reservation> for FakeRepository {
    fn create(&mut self, reservation: Reservation) -> i32 {
        self.data.insert(reservation.id, reservation);
        1
    }
}

#[test]
fn test_posts_valid_reservation_when_database_is_empty() {
    let fake_repository = RefCell::new(FakeRepository::new());

    let reservation_dto = Reservation {
        id: 1,
        name: "Katinka Ingabogovinanana".to_string(),
        email: "katinka@example.com".to_string(),
        date: NaiveDateTime::parse_from_str("2019-08-15T17:41:18.106108", "%Y-%m-%dT%H:%M:%S.%f")
            .unwrap(),
        quantity: 1,
    };
    ReservationService::new(&fake_repository).create_reservation(reservation_dto);
    assert_eq!(fake_repository.borrow().data.len(), 1);
    assert_eq!(
        fake_repository.borrow().data.get(&1).unwrap().name,
        "Katinka Ingabogovinanana"
    );
    assert_eq!(
        fake_repository.borrow().data.get(&1).unwrap().email,
        "katinka@example.com"
    );
}
