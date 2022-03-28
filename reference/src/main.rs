fn print_country(name: &mut String) {
    println!("{}", name);
}

fn main() {
    let country = String::from("Taiwan");
    let ref1 = &country;
    let ref2 = &country;

    println!("{} {}", ref1, ref2);

    // let return_ref = return_str();
    let mut my_num = 8;
    let num_ref: &mut i32 = &mut my_num;
    *num_ref += 10;
    println!("{}", num_ref);
    let num_ref1 = &my_num;
    let num_ref2 = &my_num;
    println!("{} {}", num_ref1, num_ref2);

    let country = String::from("Taiwan");
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country);

    let mut country = String::from("Taiwan");
    print_country(&mut country);
    print_country(&mut country);
}
