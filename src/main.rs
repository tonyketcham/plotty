use noise::{NoiseFn, Perlin};
use svg::node::element::Element;
use svg::{Document, Node};
fn main() {
    let mut document = Document::new().set("viewBox", (0, 0, 1000, 1000));

    let perlin = Perlin::new();

    let nodes = (100..150)
        .map(|i| {
            let f = i as f64 * 4.0;

            let x = f64::abs(perlin.get([f % 3. * 0.02, f * 0.01, 0.]) * 1000.0) + 50.;
            let y = f64::abs(perlin.get([f % 2. * 0.02, f * 0.005, 0.]) * 1000.) + 50.;

            let mut element = if f % 3. == 0. {
                let mut rect = Element::new(String::from("rect"));
                rect.assign("x", x);
                rect.assign("y", y);
                rect.assign("width", 15.);
                rect.assign("height", f % 10.);
                rect
            } else {
                let mut circle = Element::new(String::from("circle"));
                circle.assign("cx", x);
                circle.assign("cy", y);
                circle.assign("r", f % 10.);
                circle
            };

            element.assign("fill", "none");
            element.assign("stroke", "black");
            element.assign("stroke-width", 2);

            element
        })
        .collect::<Vec<_>>();

    for node in nodes {
        document = document.add(node);
    }

    svg::save("image.svg", &document).unwrap();
}
