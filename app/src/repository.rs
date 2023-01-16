// use crate::models::Reservation;

pub trait Repository<T> {
    fn create(&mut self, object: T) -> i32;
}

