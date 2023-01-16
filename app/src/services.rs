use std::cell::RefCell;

use crate::models::Reservation;
use crate::repository::Repository;

pub struct ReservationService<'a> {
    repository: &'a RefCell<dyn Repository<Reservation>>,
}

impl<'a> ReservationService<'a> {
    pub fn new(repository: &'a RefCell<dyn Repository<Reservation>>) -> Self {
        Self { repository }
    }
    pub fn create_reservation(&mut self, reservation: Reservation) -> i32 {
        self.repository.borrow_mut().create(reservation)
    }
}
