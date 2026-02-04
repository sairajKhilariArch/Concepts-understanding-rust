use std::{collections, collections::HashMap, fmt::Debug, fmt::Display};

pub trait Accommodations {
    fn book(&mut self, name: &str, night: u32);
}
pub trait Description {
    fn get_description(&self) -> String {
        format!("A wonderful place to stay")
    }
}

// -----------------------------------------------------------
// ---------------------------------HOTEL---------------------
// -----------------------------------------------------------

#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservations: collections::HashMap<String, u32>,
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
        format!(" {} : {} ", self.name, self.get_description())
    }
}

impl<T> Accommodations for Hotel<T> {
    fn book(&mut self, name: &str, night: u32) {
        self.reservations.insert(name.to_string(), night);
    }
}

impl<T: Display> Description for Hotel<T> {
    fn get_description(&self) -> String {
        format!(" plz enjoy  {}'s Room", self.name)
    }
}

// -----------------------------------------------------------
// ---------------------------------AIRBNB--------------------
// -----------------------------------------------------------

#[derive(Debug)]
pub struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnb {
    pub fn new(name: &str) -> Self {
        Self {
            host: name.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodations for AirBnb {
    fn book(&mut self, name: &str, night: u32) {
        self.guests.push((name.to_string(), night))
    }
}

impl Description for AirBnb {
    fn get_description(&self) -> String {
        format!(" plz enjoy  {}'s apartment", self.host)
    }
}





