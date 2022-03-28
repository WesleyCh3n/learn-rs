fn take_fifth(val:& Vec<i32>) -> Option<i32> {
    if val.len() < 5 {
        None
    } else {
        Some(val[4])
    }
}

fn handle_option(opt: Vec<Option<i32>>) {
    for item in opt {
        match item {
            Some(num) => println!("Found a {}!", num),
            None => println!("Found None"),
        }
    }
}

fn give_result(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        return Ok(())
    } else {
        return Err(())
    }
}

fn check_if_five(input: i32) -> Result<i32, String> {
    match input {
        5 => Ok(input),
        _ => Err("Wrong".into())
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let big_vec = vec![1, 2, 3, 4, 5];
    let mut opt = Vec::new();
    opt.push(take_fifth(&new_vec));
    opt.push(take_fifth(&big_vec));
    handle_option(opt);

    if take_fifth(&big_vec).is_some() {
        println!("Yes");
    };

    if give_result(12).is_ok() {
        println!("Ok")
    } else {
        println!("Error")
    }

    let mut reslt_vec = Vec::new();
    for num in 2..8 {
        reslt_vec.push(check_if_five(num));
    }

    println!("{:?}", reslt_vec);
    let my_vec = vec![2, 3, 4];

    for i in 0..10 {
        match my_vec.get(i) {
            Some(num) => println!("Number: {}", num),
            None => {},
        }
    }

    for i in 0..10 {
        if let Some(num) = my_vec.get(i) {
            println!("Number: {}", num)
        }
    }
}
