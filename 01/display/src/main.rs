use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: i64,
    y: i64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}i", self.real, if self.imag >= 0. { "+" } else { "-" }, self.imag)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: ", count)?;
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let minmax = MinMax(1,2);
    println!("{}", minmax);

    let pt = Point2D{x: 3, y: 4};
    println!("{}", pt);
    println!("{:?}", pt);

    let cpl = Complex{real: 3.3, imag: 7.2};
    println!("{}", cpl);
    println!("{:?}", cpl);

    let v = List(vec![1,2,3]);
    println!("{}", v);
    println!("{:?}", v);
}
