enum ThingsInSky {
    Sun(String),
    Stars(String),
}

fn return_number<T>(number: T) -> T
where
    T: std::fmt::Display,
{
    println!("{}", number);
    number
}

fn compare_and_display<T, U>(msg: T, num1: U, num2: U)
where
    T: std::fmt::Display,
    U: std::fmt::Display + std::cmp::PartialOrd
{
    println!("{}! Is {} greater than {}? {}", msg, num1, num2, num1 > num2);
}

fn main() {
    let child = 5;
    let married = true;

    /* match guard */
    match (child, married) {
        (child, married) if !married => {
            println!("married: {}, child: {}", married, child)
        }
        (child, married) if married => {
            println!("married: {}, child: {}", married, child)
        }
        _ => println!("married: {}, child: {}", married, child),
    }

    let num = 12;
    match num {
        number @ 4 => println!("{}", number),
        number @ 12 => println!("{}", number),
        _ => println!("oh oh"),
    }

    let time = 8;
    use ThingsInSky::*;
    let skystate = match time {
        6..=18 => Sun("Sun".into()),
        _ => Stars("Stars".into()),
    };

    match skystate {
        Sun(des) => println!("{}", des),
        Stars(des) => println!("{}", des),
    }

    let mut cnt1 = 0;
    let mut cnt2 = 0;

    'first_loop: loop {
        cnt1 += 1;
        println!("cnt1: {}", cnt1);
        if cnt1 > 9 {
            'sec_loop: loop {
                cnt2 += 1;
                println!("cnt2: {}", cnt2);
                if cnt2 == 3 {
                    break 'first_loop;
                } else {
                    continue 'sec_loop;
                }
            }
        }
    }

    let mut cnt = 5;
    let my_num = loop {
        cnt += 1;
        if cnt % 7 == 1 {
            break cnt;
        }
    };
    println!("{}", my_num);

    let _number: i32 = return_number(5);
    compare_and_display("[INFO]", 9, 8);
}
