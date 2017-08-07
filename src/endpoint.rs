use std::fmt::Debug;

use polymorphic::{Record, Entry};

pub trait Dispenser {
    type Item;
    
    fn dispense(&self) -> Self::Item;
    fn store(&mut self, i: Self::Item);
}

pub struct Simple<T> {
    data: T,
}

impl<T: Clone> Simple<T> {
    pub fn new(d: T) -> Simple<T> {
        Simple {
            data: d,
        }
    }
}

impl<T: Clone> Dispenser for Simple<T> {
    type Item = T;

    fn dispense(&self) -> Self::Item {
        self.data.clone()
    }

    fn store(&mut self, i: Self::Item) {
        self.data = i;
    }
}

pub struct Zingle;

impl Zingle {
    pub fn zingle(&mut self, rec: &Record) {
        println!("ZINGLE! {}", &rec.text());
    }
}

pub struct Bongle;

impl Bongle {
    pub fn bongle(&mut self) {
        println!("BONGLE!");
    }
}

/*
pub struct Generic<T> {
    data: T,
}

impl<T: Debug + Copy + Send> Generic<T> {
    pub fn new(d: T) -> Generic<T> {
        Generic {
            data: d,
        }
    }

    pub fn get(&self) -> T {
        self.data
    }

    pub fn set(&mut self, d: T) {
        self.data = d;
    }
}
*/
