use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::iter::FromIterator;

pub fn print_pdf() {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(215.9), Mm(279.4), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let points1 = vec![(Point::new(Mm(10.0), Mm(10.0)), false),
                   (Point::new(Mm(10.0), Mm(269.4)), false),
                   (Point::new(Mm(205.9), Mm(269.4)), false),
                   (Point::new(Mm(205.9), Mm(10.0)), false)];

    let line1 = Line {
    points: points1,
    is_closed: true,
    has_fill: false,
    has_stroke: true,
    is_clipping_path: false,
};

current_layer.add_shape(line1);
    let text = "MSI - Chemical Processes";
    let mut font_reader = std::io::Cursor::new(include_bytes!("../assets/fonts/RobotoMedium.ttf").as_ref());
    let font = doc.add_external_font(&mut font_reader).unwrap();

    current_layer.use_text(text, 16.0, Mm(25.0), Mm(254.4), &font);

    doc.save(&mut BufWriter::new(
        File::create("test_working.pdf").unwrap(),
    ))
    .unwrap();
}
