use tendril::{fmt::UTF8, Atomic, Tendril};

pub type SharedString = Tendril<UTF8, Atomic>;
