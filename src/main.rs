/**
 * Assignment - 1
 *
 * Make a list of items and price_per_item, and calculate the items price in `calculate_bill()` function and print it in `main()` function.
 */

fn main() {
    let items = ["Macbook", "Dell", "HP", "Lenovo"];
    let price_per_item = [1000u16, 800, 850, 670]; // Prices in dollar

    // Alice bought 1 macbook and 1 HP laptop
    // Now calculate the bill
    let total_bill = calculate_bill(
        price_per_item[0], // Macbook Price
        price_per_item[2], // HP Price
    );

    println!("Total Bill: {}", total_bill);
}

fn calculate_bill(price1: u16, price2: u16) -> u16 {
    price1 + price2
}
