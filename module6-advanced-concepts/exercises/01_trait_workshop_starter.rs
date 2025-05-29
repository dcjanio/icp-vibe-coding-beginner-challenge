use std::f64::consts::PI;

// Define the Shape trait with methods for area and perimeter calculation
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    
    // Default method to return the shape's name
    fn name(&self) -> &str {
        "Unknown Shape"
    }
}

// Implement Circle
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
    
    fn name(&self) -> &str {
        "Circle"
    }
}

// Implement Rectangle
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    fn name(&self) -> &str {
        "Rectangle"
    }
}

// Implement Triangle
struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}

impl Triangle {
    // Constructor that validates if the sides can form a valid triangle
    fn new(side_a: f64, side_b: f64, side_c: f64) -> Option<Self> {
        // Check if the sides can form a valid triangle
        if side_a + side_b > side_c && 
           side_a + side_c > side_b && 
           side_b + side_c > side_a {
            Some(Self { side_a, side_b, side_c })
        } else {
            None
        }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Heron's formula
        let s = (self.side_a + self.side_b + self.side_c) / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }
    
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
    
    fn name(&self) -> &str {
        "Triangle"
    }
}

// Function to print shape information using trait bounds
fn print_shape_info<T: Shape>(shape: &T) {
    println!("Shape: {}", shape.name());
    println!("Area: {:.2}", shape.area());
    println!("Perimeter: {:.2}", shape.perimeter());
    println!();
}

fn main() {
    // Create instances of each shape
    let circle = Circle::new(5.0);
    let rectangle = Rectangle::new(3.0, 5.0);
    
    // Create a triangle with sides 3.0, 4.0, and 5.0
    let triangle = Triangle::new(3.0, 4.0, 5.0).expect("Invalid triangle dimensions");
    
    // Print information for each shape using print_shape_info
    print_shape_info(&circle);
    print_shape_info(&rectangle);
    print_shape_info(&triangle);
    
    // Store shapes in a vector of trait objects and iterate through them
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle::new(5.0)),
        Box::new(Rectangle::new(3.0, 5.0)),
        Box::new(Triangle::new(3.0, 4.0, 5.0).unwrap())
    ];
    
    println!("Shapes in collection:");
    for shape in &shapes {
        println!("{} - Area: {:.2}, Perimeter: {:.2}", 
                 shape.name(), shape.area(), shape.perimeter());
    }
    
    // Bonus: Calculate the total area of all shapes
    let total_area: f64 = shapes.iter().map(|shape| shape.area()).sum();
    println!("\nTotal area of all shapes: {:.2}", total_area);
}