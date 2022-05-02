fn main() {
    use svg::node::element::path::Data;
    use svg::node::element::Path;
    use svg::Document;

    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "orange")
        .set("stroke-width", 1)
        .set("stroke-linecap", "round")
        .set("stroke-linejoin", "round")
        .set("d", data);

    let document = Document::new().set("viewBox", (0, 0, 70, 70)).add(path);

    svg::save("image.svg", &document).unwrap();
}
