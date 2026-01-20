use std::io;

fn main() {
    let subjects = [
        "Operating System",
        "Computer Network",
        "Data Structure",
        "Blockchain",
        "Cryptography",
    ];

    let mut marks: Vec<u32> = Vec::new();
    let mut backlogs: Vec<&str> = Vec::new();

    println!("🎓 Student Result Calculator 🦀");
    println!("====================================");
    println!("📚 Enter marks out of 100:\n");

    // 📥 Taking input for each subject
    for subject in subjects.iter() {
        loop {
            println!("👉 {} :", subject);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Input failed");

            let value: Result<u32, _> = input.trim().parse();

            match value {
                Ok(m) if m <= 100 => {
                    marks.push(m);
                    if m < 33 {
                        backlogs.push(subject);
                    }
                    break;
                }
                _ => {
                    println!("❌ Please enter valid marks (0–100)\n");
                }
            }
        }
    }

    // 🧮 Calculations
    let total: u32 = marks.iter().sum();
    let percentage = total as f32 / subjects.len() as f32;
    let sgpa = percentage / 10.0;
    let cgpa = sgpa; // simplified assumption

    let division = if percentage >= 60.0 {
        "🏆 First Division"
    } else if percentage >= 45.0 {
        "🥈 Second Division"
    } else if percentage >= 33.0 {
        "🥉 Third Division"
    } else {
        "❌ Fail"
    };

    let status = if backlogs.is_empty() {
        "✅ PASS"
    } else {
        "❌ FAIL"
    };

    // 📊 Result Table
    println!("\n📊 RESULT CARD");
    println!("====================================");
    println!("{:<25} | {:<5} | Status", "Subject", "Marks");
    println!("------------------------------------");

    for i in 0..subjects.len() {
        let result = if marks[i] < 33 {
            "❌ Back"
        } else {
            "✅ Pass"
        };
        println!("{:<25} | {:<5} | {}", subjects[i], marks[i], result);
    }

    println!("====================================");

    // 🧾 Summary
    println!("\n🧾 SUMMARY");
    println!("------------------------------------");
    println!("📌 Total Marks : {}", total);
    println!("📌 Percentage : {:.2}%", percentage);
    println!("📌 SGPA       : {:.2}", sgpa);
    println!("📌 CGPA       : {:.2}", cgpa);
    println!("📌 Division   : {}", division);
    println!("📌 Result     : {}", status);

    if !backlogs.is_empty() {
        println!("\n⚠️ Backlogs in:");
        for subject in backlogs {
            println!("❌ {}", subject);
        }
    }

    println!("\n🙏 Best of luck for your future!");
}
