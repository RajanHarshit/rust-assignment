#[derive(Debug, Clone)]
pub struct Seat {
    pub seat_number: usize,
    pub passenger_name: String,
}

#[derive(Debug)]
pub enum BookingError {
    SeatAlreadyTaken,
    SeatInvalid,
}
