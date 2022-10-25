use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::iter::FromIterator;

pub fn print_pdf() {
    // Labeling and creating document size with mm and layer
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(215.9), Mm(279.4), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Creates frames and lines for the document
    let outerframe_points = vec![
        (Point::new(Mm(10.0), Mm(10.0)), false),
        (Point::new(Mm(10.0), Mm(270.0)), false),
        (Point::new(Mm(206.0), Mm(270.0)), false),
        (Point::new(Mm(206.0), Mm(10.0)), false),
    ];

    let header_points = vec![
        (Point::new(Mm(10.0), Mm(254.0)), false),
        (Point::new(Mm(206.0), Mm(254.0)), false),
    ];

    let work_instruction_points = vec![
        (Point::new(Mm(164.8), Mm(254.0)), false),
        (Point::new(Mm(164.8), Mm(26.0)), false),
    ];

    let footer_points = vec![
        (Point::new(Mm(10.0), Mm(26.0)), false),
        (Point::new(Mm(206.0), Mm(26.0)), false),
    ];

    // Draws frames and lines
    let outerframe = Line {
        points: outerframe_points,
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    let header = Line {
        points: header_points,
        is_closed: false,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    let instruction_area = Line {
        points: work_instruction_points,
        is_closed: false,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    let footer_area = Line {
        points: footer_points,
        is_closed: false,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    // Applies frames and lines
    current_layer.add_shape(outerframe);
    current_layer.add_shape(header);
    current_layer.add_shape(instruction_area);
    current_layer.add_shape(footer_area);

    let text = "MSI - Chemical Processes";
    let mut font_reader =
        std::io::Cursor::new(include_bytes!("../assets/fonts/Roboto-Regular.ttf").as_ref());
    let font = doc.add_external_font(&mut font_reader).unwrap();

    // text, font size, x from left edge, y from bottom edge, font
    current_layer.use_text(text, 16.0, Mm(85.0), Mm(262.0), &font);
    let text1 = "MSI - Chemical Processes Body";

    current_layer.use_text(text1, 12.0, Mm(25.0), Mm(154.4), &font);
    doc.save(&mut BufWriter::new(
        File::create("test_working.pdf").unwrap(),
    ))
    .unwrap();
}
