/**
 * Assignment - 2: Take marks of student and calculate the total and average of marks
 */

fn main() {
    let marks1 = 80;
    let marks2 = 65;

    let (total, average) = calculate_marks(marks1, marks2);

    println!("Total Marks    : {}", total);
    println!("Average Marks  : {}", average);
}

fn calculate_marks(marks1: i32, marks2: i32) -> (i32, i32) {
    let total = marks1 + marks2;
    let average = total / 2;

    // tuple return
    (total, average)
}
