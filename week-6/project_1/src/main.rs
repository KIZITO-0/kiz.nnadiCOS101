use std::io;

fn main() {
    println!("====== RESTAURANT MENU ======");
    println!("P - Poundo Yam / Edinkainko Soup : ₦3200");
    println!("F - Fried Rice & Chicken        : ₦3000");
    println!("A - Amala & Ewedu Soup          : ₦2500");
    println!("E - Eba & Egusi Soup            : ₦2000");
    println!("W - White Rice & Stew           : ₦2500");

    // Input food type
    println!("\nEnter food code (P, F, A, E, W):");
    let mut food = String::new();
    
       io::stdin().read_line(&mut food)
        .expect("Failed to read input");

    let food = food.trim().to_uppercase();

    // Input quantity
    println!("Enter quantity:");
    let mut qty = String::new();
    io::stdin()
        .read_line(&mut qty)
        .expect("Failed to read input");

    let qty: u32 = qty.trim().parse().expect("Enter a valid number");

    // Determine price
    let price = match food.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food code!");
            return;
        }
    };

    let mut total = price * qty;

    // Apply discount if total > 10000
    if total > 10000 {
        let discount = (total as f64) * 0.05;
        let final_amount = (total as f64) - discount;

        println!("Total Cost: ₦{}", total);
        println!("Discount (5%): ₦{:.2}", discount);
        println!("Amount Payable: ₦{:.2}", final_amount);
    } else {
        println!("Amount Payable: ₦{}", total);
    }
}

