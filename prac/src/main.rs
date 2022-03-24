struct Number {
    odd: bool,
    value: i32,
}

struct NumRef<'a> {
    x: &'a i32,
}

struct Person {
    name: &'static str,
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

// when single input lifetime, no need to specifty lifetime
fn number_value<'a>(n: &'a Number) -> &'a i32 {
    &n.value
}

fn compare<T>(lhs: T, rhs: T)
where
    T: std::fmt::Debug + PartialEq,
{
    println!("{:?} {} {:?}", lhs, if lhs == rhs { "==" } else { "!=" }, rhs)
}

fn tail(s: &[u8]) -> &[u8] {
    &s[1..]
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

    // NOTE: Option unwrap if something return it's content, nothing it panic
    // NOTE: Result unwrap if OK return it's content, err it panic
    let o1: Option<i32> = Some(12);
    let a = o1.unwrap();
    println!("a: {}", a);

    let o1: Option<i32> = None;
    // let a = o1.unwrap();
    println!("o1: {:?}, a: {}",o1, a);

    let mut n = Number { value: 47, odd: false };
    {
        let v = number_value(&n);
        println!("{}", v);
    }
    n = Number{ value: 12, ..n };
    println!("{}", n.value);

    let x = 12;
    let a = NumRef{ x: &x };
    println!("{}", a.x);

    let p = Person {
        name: "Wesley"
    };
    println!("p: {}", p.name);

    // Error
    /* let name = format!("Wesley {}", "Chen"); */
    /* p = Person { name: &name }; */

    // silice
    assert!((0..).contains(&100000));
    assert!(!(..20).contains(&20));
    assert!((..=20).contains(&20));
    assert!((3..6).contains(&3));

    let x = vec![1,2,3,4];
    let y = tail(&x);
    println!("y = {:?}", y);

}
