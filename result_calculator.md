# 🎓 Rust Assignment – Student Result Calculator with PDF Generation (Complete Notes)

These notes explain a **complete Rust assignment** that:

- Takes student marks subject-wise
- Calculates result, percentage, SGPA, CGPA
- Detects backlogs
- Displays a result card in terminal
- **Generates a PDF report card** using Rust

This explanation is **step-by-step**, beginner-friendly, and goes **from start to end** of the program.

---

## 🧠 Big Picture – What Does This Project Do?

This project is a **CLI-based Student Result Calculator**.

Flow of the program:

1. Show subject list
2. Take marks input safely
3. Validate marks (0–100)
4. Detect pass/fail & backlogs
5. Calculate:
   - Total marks
   - Percentage
   - SGPA & CGPA
   - Division
6. Print a clean result card in terminal
7. Ask user if they want a **PDF report card**
8. Generate `report_card.pdf`

👉 This project combines **logic + user input + file generation** — real-world Rust usage.

---

## 📁 Project Structure

```text
src/
 ├── main.rs   → main application logic
 └── pdf.rs    → PDF generation logic
Cargo.toml     → project configuration & dependencies
```

Rust encourages **modular design**, and this project follows that perfectly.

---

# 🦀 main.rs – Core Application Logic

---

## 1️⃣ Module Declaration

```rust
mod pdf;
```

- Tells Rust that a file named `pdf.rs` exists
- Allows us to use functions written inside `pdf.rs`

Think of it like:
> “Hey Rust, there is another file with helper logic.”

---

## 2️⃣ Importing Standard Input

```rust
use std::io;
```

Needed to:
- Read user input from keyboard

---

## 3️⃣ Program Entry Point

```rust
fn main() {
```

- Execution of the program starts here

---

## 4️⃣ Subjects Declaration 📚

```rust
let subjects = [
    "Operating System",
    "Computer Network",
    "Data Structure",
    "Blockchain",
    "Cryptography",
];
```

- Array of subject names
- Fixed size (5 subjects)
- Used for input, display, and PDF generation

---

## 5️⃣ Marks & Backlogs Storage 🧮

```rust
let mut marks: Vec<u32> = Vec::new();
let mut backlogs: Vec<&str> = Vec::new();
```

### Why `Vec`?

- Number of marks grows dynamically
- `Vec` allows push operations

### Why `&str` for backlogs?

- We store references to subject names
- Subjects already exist in memory

---

## 6️⃣ Welcome Message 🖨️

```rust
println!("🎓 Student Result Calculator 🦀");
```

Purely for **user experience**.

---

## 7️⃣ Taking Input for Each Subject 🔁

```rust
for subject in subjects.iter() {
    loop {
        ...
    }
}
```

### Why nested loop?

- Outer loop → iterate over subjects
- Inner loop → repeat input until valid marks entered

---

## 8️⃣ Reading and Validating Marks 🛡️

```rust
let value: Result<u32, _> = input.trim().parse();
```

This is **safe parsing**.

### Match Handling

```rust
match value {
    Ok(m) if m <= 100 => { ... }
    _ => { println!("❌ Please enter valid marks"); }
}
```

What this ensures:

- Only numbers allowed
- Only marks from 0–100 allowed
- Invalid input → retry

---

## 9️⃣ Backlog Detection 🚨

```rust
if m < 33 {
    backlogs.push(subject);
}
```

Rule:
- Marks < 33 → backlog

Subjects with backlogs are stored for later use.

---

## 🔟 Total Marks Calculation

```rust
let total: u32 = marks.iter().sum();
```

- `iter()` → iterate over marks
- `sum()` → built-in iterator method

---

## 1️⃣1️⃣ Percentage, SGPA, CGPA 📊

```rust
let percentage = total as f32 / subjects.len() as f32;
let sgpa = percentage / 10.0;
let cgpa = sgpa;
```

### Type Casting

- `as f32` used to avoid integer division

---

## 1️⃣2️⃣ Division Logic 🏆

```rust
let division = if percentage >= 60.0 { ... }
```

Based on percentage:

| Percentage | Division |
|---------|----------|
| ≥ 60 | First |
| ≥ 45 | Second |
| ≥ 33 | Third |
| < 33 | Fail |

---

## 1️⃣3️⃣ Final Pass / Fail Status

```rust
let status = if backlogs.is_empty() { "PASS" } else { "FAIL" };
```

No backlog = PASS

---

## 1️⃣4️⃣ Printing Result Table 📋

```rust
for i in 0..subjects.len() { ... }
```

Displays:
- Subject
- Marks
- Pass / Backlog

---

## 1️⃣5️⃣ Summary Section 🧾

Shows:
- Total marks
- Percentage
- SGPA
- CGPA
- Division
- Result status

---

## 1️⃣6️⃣ Asking for PDF Download 📥

```rust
println!("Do you want to download report card as PDF? (y/n)");
```

User choice decides next step.

---

## 1️⃣7️⃣ Calling PDF Generator

```rust
pdf::generate_report_card_pdf(...);
```

- Calls function from `pdf.rs`
- Passes all required data

---

# 📄 pdf.rs – PDF Generation Logic

---

## 1️⃣ Using `printpdf` Crate

```rust
use printpdf::*;
```

This crate allows:
- Creating PDF documents
- Writing text
- Saving files

---

## 2️⃣ Creating PDF Document 📄

```rust
PdfDocument::new("Student Report Card", Mm(210.0), Mm(297.0), "Layer 1");
```

- A4 size PDF
- Title
- Single layer

---

## 3️⃣ Font Setup 🔤

```rust
let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
```

Used for text rendering.

---

## 4️⃣ Writing Text with Coordinates 📐

```rust
layer.use_text(text, size, x, y, &font);
```

PDF uses **absolute positioning**.

---

## 5️⃣ Subject-wise Marks in PDF

```rust
for i in 0..subjects.len() { ... }
```

Same logic reused from terminal output.

---

## 6️⃣ Summary Section in PDF 🧾

Writes:
- Percentage
- SGPA
- CGPA
- Division
- Result

---

## 7️⃣ Writing Backlogs

Only printed if backlogs exist.

---

## 8️⃣ Saving PDF File 💾

```rust
File::create("report_card.pdf");
doc.save(...);
```

Creates PDF file in project directory.

---

# 📦 Cargo.toml – Dependency Management

```toml
[dependencies]
printpdf = "0.7"
```

Tells Cargo:
- Download `printpdf` crate
- Make it available in code

---

## 🎯 Key Rust Concepts Used

- Modules
- Vectors
- Pattern matching
- Iterators
- Error handling with `Result`
- Type casting
- External crates
- File generation

---

## ✅ Why This Is a Great Assignment

- Real-world logic
- Clean modular design
- Safe input handling
- Practical file output

---

## 🧠 Final Summary

This assignment shows how Rust can be used to:

- Build real CLI tools
- Handle user input safely
- Perform calculations
- Generate files (PDF)

After understanding this project, you are ready for:
- CSV export
- Database storage
- Web-based result systems

---

**End of Assignment Notes – Student Result Calculator 🦀📘**

