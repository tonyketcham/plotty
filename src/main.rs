use noise::{NoiseFn, Perlin};
use svg::node::element::Element;
use svg::node::Value;
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

                set_element_properties(
                    &mut rect,
                    vec![("x", x), ("y", y), ("width", 15.), ("height", f % 10.)],
                );

                rect
            } else {
                let mut circle = Element::new(String::from("circle"));

                set_element_properties(&mut circle, vec![("cx", x), ("cy", y), ("r", f % 10.)]);

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

/// Set many property attributes of an element, all at once.
fn set_element_properties<T, U>(element: &mut Element, properties: Vec<(T, U)>)
where
    T: Into<String>,
    U: Into<Value>,
{
    for (name, value) in properties {
        element.assign(name, value);
    }
}
