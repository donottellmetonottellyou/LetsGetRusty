#![allow(unused)]

trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&mut self, color: String) {
        println!("painting object {color}");
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}
impl Paint for Car {}
impl Park for Car {
    fn park(&self) {
        println!("parking car!");
    }
}

struct Truck {
    info: VehicleInfo,
}
impl Truck {
    fn unload(&self) {
        println!("unloading truck");
    }
}
impl Paint for Truck {}
impl Park for Truck {
    fn park(&self) {
        println!("parking truck!")
    }
}

struct House;
impl Paint for House {
    fn paint(&mut self, color: String) {
        println!("Painting house {color}");
    }
}

fn main() {}
