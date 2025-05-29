// Define module structure for e-commerce system

mod product {
    // Product struct and implementation
    #[derive(Debug, Clone)]
    pub struct Product {
        pub id: u32,
        pub name: String,
        pub price: f64,
        pub description: String,
    }

    impl Product {
        pub fn new(id: u32, name: &str, price: f64, description: &str) -> Self {
            Product {
                id,
                name: name.to_string(),
                price,
                description: description.to_string(),
            }
        }

        pub fn display(&self) {
            println!("Product: {} (ID: {})", self.name, self.id);
            println!("  Price: ${:.2}", self.price);
            println!("  Description: {}", self.description);
        }
    }
}

mod user {
    // User struct and implementation
    #[derive(Debug, Clone)]
    pub struct User {
        pub id: u32,
        pub name: String,
        pub email: String,
        pub address: String,
    }

    impl User {
        pub fn new(id: u32, name: &str, email: &str, address: &str) -> Self {
            User {
                id,
                name: name.to_string(),
                email: email.to_string(),
                address: address.to_string(),
            }
        }

        pub fn display(&self) {
            println!("User: {} (ID: {})", self.name, self.id);
            println!("  Email: {}", self.email);
            println!("  Address: {}", self.address);
        }
    }
}

mod order {
    use crate::product::Product;
    use crate::user::User;

    // Order status enum
    #[derive(Debug, Clone)]
    pub enum OrderStatus {
        Pending,
        Shipped,
        Delivered,
        Cancelled,
    }

    // Order struct and implementation
    #[derive(Debug)]
    pub struct Order {
        pub id: u32,
        pub user: User,
        pub items: Vec<(Product, u32)>, // (Product, quantity)
        pub status: OrderStatus,
        pub total: f64,
    }

    impl Order {
        pub fn new(id: u32, user: User, items: Vec<(Product, u32)>) -> Self {
            let total = items.iter().fold(0.0, |acc, (product, quantity)| {
                acc + product.price * (*quantity as f64)
            });

            Order {
                id,
                user,
                items,
                status: OrderStatus::Pending,
                total,
            }
        }

        pub fn update_status(&mut self, status: OrderStatus) {
            self.status = status;
        }

        pub fn display(&self) {
            println!("Order: #{}", self.id);
            println!("Status: {:?}", self.status);
            
            println!("Customer Information:");
            self.user.display();
            
            println!("Items:");
            for (product, quantity) in &self.items {
                println!("  {}x {} - ${:.2} each", quantity, product.name, product.price);
            }
            
            println!("Total: ${:.2}", self.total);
        }
    }
}

mod inventory {
    use std::collections::HashMap;
    use crate::product::Product;

    pub struct Inventory {
        stock: HashMap<u32, u32>, // product_id -> quantity
    }

    impl Inventory {
        pub fn new() -> Self {
            Inventory {
                stock: HashMap::new(),
            }
        }

        pub fn add_product(&mut self, product: &Product, quantity: u32) {
            *self.stock.entry(product.id).or_insert(0) += quantity;
        }

        pub fn remove_product(&mut self, product_id: u32, quantity: u32) -> Result<(), &'static str> {
            let current_quantity = self.stock.get(&product_id).unwrap_or(&0);
            
            if *current_quantity < quantity {
                return Err("Not enough items in stock");
            }
            
            if let Some(qty) = self.stock.get_mut(&product_id) {
                *qty -= quantity;
                if *qty == 0 {
                    self.stock.remove(&product_id);
                }
            }
            
            Ok(())
        }

        pub fn check_stock(&self, product_id: u32) -> u32 {
            *self.stock.get(&product_id).unwrap_or(&0)
        }

        pub fn display(&self) {
            println!("Current Inventory:");
            for (product_id, quantity) in &self.stock {
                println!("  Product ID: {} - Quantity: {}", product_id, quantity);
            }
        }
    }
}

// Import necessary items for main
use product::Product;
use user::User;
use order::{Order, OrderStatus};
use inventory::Inventory;

fn main() {
    // 1. Create some products
    let product1 = Product::new(1, "Laptop", 999.99, "High-performance laptop");
    let product2 = Product::new(2, "Mouse", 24.99, "Wireless ergonomic mouse");
    let product3 = Product::new(3, "Monitor", 149.99, "24-inch HD monitor");
    
    // 2. Add products to inventory
    let mut inventory = Inventory::new();
    inventory.add_product(&product1, 5);
    inventory.add_product(&product2, 10);
    inventory.add_product(&product3, 3);
    
    println!("Initial inventory:");
    inventory.display();
    
    // 3. Create a user
    let user = User::new(1, "John Doe", "john@example.com", "123 Main St, Anytown");
    
    // 4. Create an order for the user with some products
    let order_items = vec![
        (product1.clone(), 1),
        (product2.clone(), 2),
    ];
    
    let mut order = Order::new(1, user.clone(), order_items);
    
    // Update inventory
    inventory.remove_product(1, 1).unwrap();
    inventory.remove_product(2, 2).unwrap();
    
    // 5. Print order details
    println!("\nOrder created:");
    order.display();
    
    // Update order status
    order.update_status(OrderStatus::Shipped);
    
    println!("\nUpdated order status:");
    order.display();
    
    // Check updated inventory
    println!("\nUpdated inventory:");
    inventory.display();
}