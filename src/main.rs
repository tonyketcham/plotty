use noise::{NoiseFn, Perlin};
use svg::node::element::Circle;
use svg::Document;

fn main() {
    let mut document = Document::new().set("viewBox", (0, 0, 300, 2120));

    let perlin = Perlin::new();

    let circles = (0..300)
        .map(|i| {
            let f = i as f64;
            let x = f * perlin.get([f * 0.001, 1., 0.05 * f]) * 7.;
            let y = f * 7.;
            Circle::new()
                .set("cx", x)
                .set("cy", y + 10.)
                .set("r", 5. + perlin.get([x, y, 0.0]) * 10.)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 2)
        })
        .collect::<Vec<_>>();

    for circle in circles {
        document = document.add(circle);
    }

    svg::save("image.svg", &document).unwrap();
}

// #[derive(Clone)]
// struct Sketch {
//     data: Data,
//     path: Path,
//     document: SVG,
// }

// impl Sketch {
//     fn line_loop(&self) {
//         self.data.line_by((0, 5));
//     }
// }
