#![allow(unused)]

trait Vehicle: Paint {
    fn park(&self);

    fn get_default_color() -> String {
        "black".into()
    }
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
impl Vehicle for Car {
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
impl Vehicle for Truck {
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

fn main() {
    let mut car = Car {
        info: VehicleInfo {
            make: "Honda".into(),
            model: "Civic".into(),
            year: 1995,
        },
    };
    let mut house = House {};
    let mut object = create_paintable_object(true);

    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

    paint_red(&mut car);
    paint_red(&mut house);
    paint_red(object.as_mut());

    paint_vehicle_red(&mut car);
    // paint_vehicle_red(&mut house);
    // paint_vehicle_red(&mut object);
}

fn paint_red(object: &mut dyn Paint) {
    object.paint("red".into());
}

fn paint_red2(object: &mut impl Paint) {
    object.paint("red".into());
}

fn paint_vehicle_red<T>(object: &mut T)
where
    T: Vehicle,
{
    object.paint("red".into());
}

fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {
            info: VehicleInfo {
                make: "Honda".into(),
                model: "Civic".into(),
                year: 1995,
            },
        })
    } else {
        Box::new(House {})
    }
}
