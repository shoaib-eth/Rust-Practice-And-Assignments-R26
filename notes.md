# 🦀 Rust Assignment – Interactive Store Program (Complete Line‑by‑Line Explanation)

These notes explain the given Rust program **from start to end**, in a **very beginner‑friendly and detailed way**.

Goal 🎯:
- Understand **how the program works internally**
- Understand **why each line is written**
- Understand **Rust concepts used in this assignment**

This is written so that even a **first‑time Rust learner** can follow it easily.

---

## 1️⃣ What Does This Program Do? 🛒

This program simulates a **small shopping system**:

1. Shows a list of items with prices
2. Asks the user to choose an item
3. Asks the user to enter quantity
4. Calculates the total bill
5. Prints a final bill summary

In simple words:
> 🛍️ A mini **console‑based store application** written in Rust

---

## 2️⃣ Importing Standard Input Module 📥

```rust
use std::io;
```

### Why is this needed?

- Rust does NOT allow reading user input by default
- `std::io` provides tools to:
  - Read input from keyboard
  - Work with input/output streams

Without this line ❌:
- `stdin()` would not be available

---

## 3️⃣ Program Entry Point – `main()` 🚀

```rust
fn main() {
```

- `main` is the **starting point** of every Rust program
- Code execution starts here

---

## 4️⃣ Declaring Items and Prices 🛒💰

```rust
let items = ["MacBook 💻", "IPhone 📱", "Desktop 🖥️", "Keyboard ⌨️"];
let prices = [1000u32, 800, 850, 70];
```

### What is happening here?

- Two **arrays** are created
- `items` stores product names
- `prices` stores product prices

Important points:
- Both arrays have the **same length**
- Index `i` in `items` corresponds to index `i` in `prices`

Example:
```text
items[0]  → "MacBook 💻"
prices[0] → 1000
```

---

## 5️⃣ Printing Welcome Message 🖨️

```rust
println!("🛍️ Welcome to Rusty Electronics Store 🦀");
println!("=================================");
println!("📋 Available Items:\n");
```

- `println!` prints text to the console
- `\n` adds a new line

This improves **user experience and readability**.

---

## 6️⃣ Displaying Item List Using Loop 🔁

```rust
for i in 0..items.len() {
    println!(
        "{}️⃣ {} - 💲{}",
        i + 1,
        items[i],
        prices[i]
    );
}
```

### Step‑by‑step explanation:

- `items.len()` gives number of items (here: 4)
- `0..items.len()` creates a range: `0,1,2,3`
- `i` is the index

### Why `i + 1`?

- Arrays are **0‑indexed**
- Users think in **1‑based indexing**

So:
```text
User sees: 1 → MacBook
Program uses: index 0
```

---

## 7️⃣ Asking User to Select Item 🧠

```rust
println!("\n👉 Enter item number (1-4): ");
```

This tells the user:
- Choose an item using numbers shown

---

## 8️⃣ Reading Item Input from User 📥

```rust
let mut item_input = String::new();
io::stdin().read_line(&mut item_input).expect("Failed to read input");
```

### Important concepts:

- `String::new()` creates an empty string
- `mut` is required because input modifies the string
- `read_line` stores user input inside `item_input`

If input fails:
- Program crashes with message

---

## 9️⃣ Converting User Input into Index 🔢

```rust
let item_index: usize = item_input
    .trim()
    .parse::<u32>()
    .expect("Please enter a number") as usize - 1;
```

### Breakdown:

1. `trim()` removes whitespace and newline
2. `parse::<u32>()` converts string → number
3. `as usize` converts to array index type
4. `- 1` adjusts for zero‑based indexing

Example:
```text
User enters: 2
Index becomes: 1
```

---

## 🔟 Asking for Quantity 📦

```rust
println!("👉 Enter quantity: ");
```

Simple prompt asking:
- How many items user wants

---

## 1️⃣1️⃣ Reading Quantity Input 📥

```rust
let mut quantity_input = String::new();
io::stdin().read_line(&mut quantity_input).expect("Failed to read input");

let quantity: u32 = quantity_input.trim().parse().expect("Please enter a number");
```

### What happens here?

- Reads quantity as string
- Removes whitespace
- Converts it to `u32`

Rust ensures:
- Only valid numbers are accepted

---

## 1️⃣2️⃣ Calculating Total Bill 🧮

```rust
let total_bill = calculate_bill(prices[item_index], quantity);
```

- Calls a **separate function**
- Passes:
  - price of selected item
  - quantity

This shows **modular programming**.

---

## 1️⃣3️⃣ Printing Final Bill 🧾

```rust
println!("\n🧾 BILL SUMMARY");
println!("==============================");
println!("🛒 Item     : {}", items[item_index]);
println!("📦 Quantity : {}", quantity);
println!("💰 Total    : 💲{}", total_bill);
println!("==============================");
println!("🙏 Thank you for shopping with us!");
```

This section:
- Displays a **clean receipt**
- Uses stored values

---

## 1️⃣4️⃣ Bill Calculation Function 🧮

```rust
fn calculate_bill(price_per_item: u32, quantity: u32) -> u32 {
    price_per_item * quantity
}
```

### Explanation:

- Takes two parameters:
  - `price_per_item`
  - `quantity`
- Multiplies them
- Returns the result

This function:
- Improves readability
- Makes code reusable

---

## 1️⃣5️⃣ Why This Code Is Well Written ✅

- Uses functions for logic
- Clear variable names
- User‑friendly output
- Safe input handling
- Proper data types

---

## 1️⃣6️⃣ Key Rust Concepts Used 🧠✨

- Arrays
- Loops (`for`)
- User input (`stdin`)
- Type casting
- Functions
- Ownership & borrowing (implicitly safe)

---

## Final Summary ✨

This program is a **perfect beginner assignment** that teaches:

- How to interact with users
- How to use arrays and indexing
- How to structure Rust programs
- How to calculate values safely

After understanding this code, you are ready for:
- More complex CLI programs
- File handling
- Struct‑based designs

---

**End of Assignment Notes 🦀📘**

