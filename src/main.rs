// use std::default::default;
use std::time;

// struct CapacityUnit {
//     volume: u32,
// }

struct Cup {
    name: String,
    size: u32,
    current_amount: u32,
}

impl Cup {
    fn empty(&mut self) {
        self.current_amount = 0;
    }

    fn show_current_amount(&self) {
        println!(
            "current remaining amount in '{}': {}",
            self.name, self.current_amount
        );
    }
}

#[derive(Debug)]
struct Storage {
    capacity: u32,
    current_amount: u32,
}

impl Storage {
    fn fill(&mut self) {
        self.current_amount = self.capacity;
    }
}

#[derive(Debug)]
struct WaterPurifier {
    name: String,
    vendor: String,
    crated_at: time::Instant,
    water_storage: Storage,
}

impl WaterPurifier {
    fn new(name: String) -> WaterPurifier {
        let size = 10;

        WaterPurifier {
            name,
            vendor: String::from("Abiria Inc."),
            crated_at: time::Instant::now(),
            water_storage: Storage {
                capacity: size,
                current_amount: size,
            },
        }
    }

    fn get_water(&mut self, volume: &str, cup: &mut Cup) {
        let amount: u32 = if volume == "big" {
            5
        } else if volume == "medium" {
            3
        } else if volume == "small" {
            1
        } else {
            println!("Unsupported volume: '{}'", volume);
            0
        };

        if self.water_storage.current_amount >= amount {
            self.water_storage.current_amount -= amount;
        } else {
            println!("the stored water is not enough to fill your cup.");
            println!(
                "please retry after refill the '{}' by using 're_fill_storage()' method.",
                self.name
            );

            return;
        }

        if fill_cup(amount, cup) {
            println!("Your cup is now filled!");
        } else {
            println!("Your cup is too small");
            println!("Please retry after empty the cup or with bigger one");
        }
    }

    fn re_fill_storage(&mut self) {
        self.water_storage.fill();

        println!("'{}' has refilled!", self.name);
    }

    fn show_status(&self) {
        println!("status of '{}'", self.name);
        let curr: f32 = self.water_storage.current_amount as f32;
        let size: f32 = self.water_storage.capacity as f32;

        println!("storage capacity: {}", size);
        println!("current amount of water: {}", curr);

        println!("percentage of remaining: {}%", (curr / size) * 100.0);
    }
}

fn fill_cup(amount_to_fill: u32, cup: &mut Cup) -> bool {
    let cup_size = cup.size;
    let current_amount = cup.current_amount;

    let will = current_amount + amount_to_fill;

    if (will) > cup_size {
        return false;
    }

    cup.current_amount = will;

    return true;
}

fn main() {
    let mut purifier = WaterPurifier::new(String::from("kawaii water purifier"));

    println!("{:#?}", purifier);

    purifier.show_status();

    let mut my_cup = Cup {
        name: String::from("my kawaii cup"),
        size: 2,
        current_amount: 1,
    };

    my_cup.show_current_amount();

    purifier.get_water("small", &mut my_cup);

    my_cup.show_current_amount();

    purifier.get_water("small", &mut my_cup);

    my_cup.empty();

    purifier.get_water("big", &mut my_cup);

    let mut my_another_cup = Cup {
        name: String::from("my another cup"),
        size: 7,
        ..my_cup
    };

    purifier.get_water("small", &mut my_another_cup);

    purifier.get_water("big", &mut my_another_cup);

    purifier.show_status();

    purifier.re_fill_storage();
}
