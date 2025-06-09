// Define Customer and Product Structures
struct Customer {
    name: String,
    surname: String,
    balance: f64,
}

struct Product {
    name: String,
    price: f64,
    stock_quantity: u32,
}

// Market functions
impl Customer {
    // Function for purchasing a product
    fn buy_product(&mut self, product: &mut Product, quantity: u32) -> bool {
        // Calculate total cost
        let total_cost = product.price * quantity as f64;
        
        // Check if the product is available in sufficient quantity and customer has enough balance
        if product.stock_quantity >= quantity && self.balance >= total_cost {
            // Deduct the cost from customer's balance
            self.balance -= total_cost;
            // Reduce the stock quantity
            product.stock_quantity -= quantity;
            // Return true for successful purchase
            true
        } else {
            // Return false if purchase fails
            false
        }
    }
    
    // Helper function to display customer information
    fn display_info(&self) {
        println!("Customer: {} {}, Balance: ${:.2}", self.name, self.surname, self.balance);
    }
}

impl Product {
    // Helper function to display product information
    fn display_info(&self) {
        println!("Product: {}, Price: ${:.2}, Stock: {}", self.name, self.price, self.stock_quantity);
    }
}

// Main program
fn main() {
    println!("=== Market Simulation System ===\n");
    
    let mut customer1 = Customer {
        name: String::from("John"),
        surname: String::from("Doe"),
        balance: 100.0,
    };
    
    let mut customer2 = Customer {
        name: String::from("Jane"),
        surname: String::from("Smith"),
        balance: 50.0,
    };
    
    let mut product = Product {
        name: String::from("Laptop"),
        price: 15.0,
        stock_quantity: 10,
    };
    
    // Display initial state
    println!("Initial State:");
    customer1.display_info();
    customer2.display_info();
    product.display_info();
    println!();
    
    // Customer 1's product purchase operation
    println!("Customer 1 is buying 3 units of the product...");
    if customer1.buy_product(&mut product, 3) {
        println!("Customer 1 successfully purchased the product.");
        println!("Total cost: ${:.2}", 15.0 * 3.0);
    } else {
        println!("Customer 1 couldn't purchase the product.");
        println!("Reason: Insufficient balance or stock");
    }
    
    // Display state after first purchase
    println!("\nAfter Customer 1's purchase:");
    customer1.display_info();
    product.display_info();
    println!();
    
    // Customer 2's product purchase operation
    println!("Customer 2 is buying 8 units of the product...");
    if customer2.buy_product(&mut product, 8) {
        println!("Customer 2 successfully purchased the product.");
        println!("Total cost: ${:.2}", 15.0 * 8.0);
    } else {
        println!("Customer 2 couldn't purchase the product.");
        println!("Reason: Insufficient balance or stock");
    }
    
    // Display final state
    println!("\nFinal State:");
    customer1.display_info();
    customer2.display_info();
    product.display_info();
    
    // Additional interactive user interface (bonus)
    println!("\n=== Interactive Mode ===");
    interactive_market_mode();
}

// Bonus: Interactive market interface
fn interactive_market_mode() {
    use std::io;
    
    let mut customer = Customer {
        name: String::from("Interactive"),
        surname: String::from("User"),
        balance: 200.0,
    };
    
    let mut products = vec![
        Product { name: String::from("Book"), price: 10.0, stock_quantity: 5 },
        Product { name: String::from("Phone"), price: 50.0, stock_quantity: 3 },
        Product { name: String::from("Headphones"), price: 25.0, stock_quantity: 7 },
    ];
    
    loop {
        println!("\n--- Market Menu ---");
        println!("1. View your balance");
        println!("2. View products");
        println!("3. Buy a product");
        println!("4. Exit");
        println!("Enter your choice: ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim() {
            "1" => customer.display_info(),
            "2" => {
                println!("Available products:");
                for (i, product) in products.iter().enumerate() {
                    println!("{}. {}", i + 1, product.name);
                    product.display_info();
                }
            },
            "3" => {
                println!("Enter product number (1-{}): ", products.len());
                let mut product_input = String::new();
                io::stdin().read_line(&mut product_input).expect("Failed to read line");
                
                if let Ok(product_num) = product_input.trim().parse::<usize>() {
                    if product_num > 0 && product_num <= products.len() {
                        println!("Enter quantity: ");
                        let mut quantity_input = String::new();
                        io::stdin().read_line(&mut quantity_input).expect("Failed to read line");
                        
                        if let Ok(quantity) = quantity_input.trim().parse::<u32>() {
                            if customer.buy_product(&mut products[product_num - 1], quantity) {
                                println!("Purchase successful!");
                            } else {
                                println!("Purchase failed: Insufficient balance or stock");
                            }
                        } else {
                            println!("Invalid quantity");
                        }
                    } else {
                        println!("Invalid product number");
                    }
                } else {
                    println!("Invalid input");
                }
            },
            "4" => {
                println!("Thank you for using the market system!");
                break;
            },
            _ => println!("Invalid choice, please try again"),
        }
    }
}
