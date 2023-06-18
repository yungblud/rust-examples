use std::fmt;

struct Structure(i32);

struct List(Vec<i32>);

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    img: f64,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{count}:{}", v)?;
        }

        write!(f, "]")
    }
}


fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range: {big}, the small range: {small}",
            small = small_range,
            big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex: Complex = Complex { real: 3.3, img: 7.2 };

    println!("Display: {:?}", complex);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}