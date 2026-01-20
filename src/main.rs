/**
 * Assignment - 1
 *
 * Make a list of items and price_per_item, and calculate the items price in `calculate_bill()` function and print it in `main()` function.
 */
use std::io;

fn main() {
    // 🛒 Items and their prices
    let items = ["MacBook 💻", "IPhone 📱", "Desktop 🖥️", "Keyboard ⌨️"];
    let prices = [1000u32, 800, 850, 70];

    println!("🛍️ Welcome to Rusty Electronics Store 🦀");
    println!("=================================");
    println!("📋 Available Items:\n");

    // 📜 Show item list
    for i in 0..items.len() {
        println!("{}️⃣ {} - 💲{}", i + 1, items[i], prices[i]);
    }

    println!("\n👉 Enter item number (1-4): ");

    // 📥 Read item choice
    let mut item_input = String::new();
    io::stdin()
        .read_line(&mut item_input)
        .expect("Failed to read input");

    let item_index: usize = item_input
        .trim()
        .parse::<u32>()
        .expect("Please enter a number") as usize
        - 1;

    println!("👉 Enter quantity: ");

    // 📥 Read quantity
    let mut quantity_input = String::new();
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");

    let quantity: u32 = quantity_input
        .trim()
        .parse()
        .expect("Please enter a number");

    // 🧮 Calculate bill
    let total_bill = calculate_bill(prices[item_index], quantity);

    // 🧾 Final Bill
    println!("\n🧾 BILL SUMMARY");
    println!("==============================");
    println!("🛒 Item     : {}", items[item_index]);
    println!("📦 Quantity : {}", quantity);
    println!("💰 Total    : 💲{}", total_bill);
    println!("==============================");
    println!("🙏 Thank you for shopping with us!");
}

// 🧮 Bill calculation function
fn calculate_bill(price_per_item: u32, quantity: u32) -> u32 {
    price_per_item * quantity
}
