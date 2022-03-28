use std::collections::HashMap;

struct City {
    name: String,
    population: HashMap<u32, u32>,
}

fn main() {
    let arr1 = ["one", "two"];
    println!("{:?}", arr1);
    let arr1 = [1; 10];
    println!("{:?}", arr1);
    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let _array_of_ten_slice = &array_of_ten[0..2];
    let _array_of_ten_slice = &array_of_ten[1..7];

    let mut my_vec: Vec<String> = Vec::new();

    let name1 = String::from("Wesley");
    let name2 = String::from("Alisson");
    my_vec.push(name1);
    my_vec.push(name2);

    let my_vec: Vec<_> = [1, 2, 3].into();

    let (a, b, c) = (my_vec[0], my_vec[1], my_vec[2]);
    println!("{} {} {}", a, b, c);

    let mut taiwan = City {
        name: "Taiwan".into(),
        population: HashMap::new(),
    };

    taiwan.population.insert(1372, 3_250); // insert three dates
    taiwan.population.insert(1851, 24_000);
    taiwan.population.insert(2020, 437_619);

    for (year, population) in taiwan.population {
        println!(
            "In the year {}, the city of {} had a population of {}",
            year, taiwan.name, population
        );
    }
}
