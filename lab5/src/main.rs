use std::fmt::Display;

// --- Module setup simulated in single binary ---
mod geometry {
    pub mod shapes {
        #[derive(Debug)]
        pub struct Point {
            pub x: f64,
            pub y: f64,
        }

        impl Point {
            pub fn new(x: f64, y: f64) -> Self {
                Point { x, y }
            }

            pub fn distance(&self, other: &Point) -> f64 {
                ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
            }
        }

        pub struct Polygon {
            pub vertices: Vec<Point>,
        }

        impl Polygon {
            // TODO 3: Implemented perimeter calculation
            pub fn perimeter(&self) -> f64 {
                if self.vertices.len() < 2 {
                    return 0.0;
                }
                let mut total = 0.0;
                let len = self.vertices.len();
                for i in 0..len {
                    let next = (i + 1) % len;
                    total += self.vertices[i].distance(&self.vertices[next]);
                }
                total
            }

            // TODO 4: Implemented is_closed check
            pub fn is_closed(&self) -> bool {
                self.vertices.len() >= 3
            }
        }
    }
    pub use shapes::Point;
}

// --- 5.2 Exercise A ---
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

struct Important<'a> {
    content: &'a str,
}

impl<'a> Important<'a> {
    fn summarise(&self) -> &str {
        &self.content[..self.content.len().min(80)]
    }
}

// TODO 1: Function returning slice up to first '.'
fn first_sentence<'a>(text: &'a str) -> &'a str {
    match text.find('.') {
        Some(index) => &text[..index],
        None => text,
    }
}

// --- 5.3 Exercise B ---
fn print_largest<T: PartialOrd + Display>(list: &[T]) {
    if list.is_empty() { return; }
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    println!("The largest is {}", largest);
}

// TODO 2: Generic zip_with function
fn zip_with<A, B, C, F>(a: &[A], b: &[B], f: F) -> Vec<C>
where
    F: Fn(&A, &B) -> C,
{
    a.iter().zip(b.iter()).map(|(x, y)| f(x, y)).collect()
}

use geometry::Point;
use geometry::shapes::Polygon;

fn main() {
    println!("--- 5.2 Lifetimes ---");
    let sample_text = "Rust is fast. It provides memory safety without garbage collection.";
    println!("First sentence: {:?}", first_sentence(sample_text));

    println!("\n--- 5.3 Generics & Trait Bounds ---");
    print_largest(&[34, 50, 25, 100, 65]);
    
    let a = vec![1, 2, 3];
    let b = vec![10, 20, 30];
    let zipped = zip_with(&a, &b, |x, y| x + y);
    println!("zip_with result: {:?}", zipped);

    println!("\n--- 5.4 Modules & Geometry ---");
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    println!("Distance: {:.2}", p1.distance(&p2));

    let square = Polygon {
        vertices: vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ],
    };
    println!("Polygon closed? {}", square.is_closed());
    println!("Perimeter: {:.2}", square.perimeter());
}