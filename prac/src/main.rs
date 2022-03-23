struct Number {
    odd: bool,
    value: i32,
}

/* self trait */
trait Signed {
    fn is_negative(&self) -> bool;
}

/* impl self trait on self type */
impl Signed for Number {
    fn is_negative(&self) -> bool {
        self.value < 0
    }
}

impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

/* trait Copy need Clone  */
impl std::marker::Copy for Number {}

/* impl bultin trait on self type */
impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        Number {
            value: -self.value,
            ..self
        }
    }
}

fn print_number(n: &Number) {
    println!(
        "odd: {}, value: {}",
        if n.odd { "odd" } else { "even" },
        n.value
    )
}

fn invert(n: &mut Number) {
    n.value = -n.value
}

fn compare<T>(l: T, r: T)
where
    T: std::fmt::Debug + PartialEq,
{
    println!("{:?} {} {:?}", l, if l == r { "==" } else { "!=" }, r)
}

fn main() {
    let num = Number {
        odd: true,
        value: -2,
    };
    println!("(&num).is_negative(): {}", (&num).is_negative());
    // is equilevent...
    // NOTE: when invoke trait method, the reciever is borrowed implicitly
    println!("num.is_negative(): {}", num.is_negative());

    println!("-num.value: {}", -num.value);

    print_number(&num);

    let mut num = num;
    invert(&mut num);

    let mut n = num.clone();
    n.value += 100;
    let m = n;
    let o = n;

    print_number(&n);
    print_number(&m);
    print_number(&o);

    compare("wesley", "chen");
    compare("wesley", "wesley");
}
