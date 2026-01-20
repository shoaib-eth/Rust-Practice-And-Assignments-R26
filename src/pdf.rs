use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn generate_report_card_pdf(
    subjects: &[&str],
    marks: &[u32],
    percentage: f32,
    sgpa: f32,
    cgpa: f32,
    division: &str,
    status: &str,
    backlogs: &[&str],
) {
    let (doc, page1, layer1) =
        PdfDocument::new("Student Report Card", Mm(210.0), Mm(297.0), "Layer 1");

    let layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let mut y = 270.0;

    // 🎓 Title
    layer.use_text("Student Report Card", 18.0, Mm(20.0), Mm(y), &font);
    y -= 15.0;

    // 📊 Subject-wise marks
    for i in 0..subjects.len() {
        let subject_status = if marks[i] < 33 { "Backlog" } else { "Pass" };
        let line = format!("{} : {} ({})", subjects[i], marks[i], subject_status);
        layer.use_text(line, 12.0, Mm(20.0), Mm(y), &font);
        y -= 8.0;
    }

    y -= 10.0;

    // 🧾 Summary
    layer.use_text(
        format!("Percentage : {:.2}%", percentage),
        12.0,
        Mm(20.0),
        Mm(y),
        &font,
    );
    y -= 8.0;

    layer.use_text(format!("SGPA : {:.2}", sgpa), 12.0, Mm(20.0), Mm(y), &font);
    y -= 8.0;

    layer.use_text(format!("CGPA : {:.2}", cgpa), 12.0, Mm(20.0), Mm(y), &font);
    y -= 8.0;

    layer.use_text(
        format!("Division : {}", division),
        12.0,
        Mm(20.0),
        Mm(y),
        &font,
    );
    y -= 8.0;

    layer.use_text(format!("Result : {}", status), 12.0, Mm(20.0), Mm(y), &font);

    if !backlogs.is_empty() {
        y -= 12.0;
        layer.use_text("Backlogs:", 12.0, Mm(20.0), Mm(y), &font);
        y -= 8.0;

        for subject in backlogs {
            layer.use_text(*subject, 12.0, Mm(25.0), Mm(y), &font);
            y -= 8.0;
        }
    }

    let file = File::create("report_card.pdf").unwrap();
    doc.save(&mut BufWriter::new(file)).unwrap();
}
