use noise::{NoiseFn, Perlin};
use svg::node::element::Circle;
use svg::Document;

fn main() {
    let mut document = Document::new().set("viewBox", (0, 0, 200, 900));

    let perlin = Perlin::new();

    let circles = (0..400)
        .map(|i| {
            let f = i as f64;
            let x = perlin.get([f * 0.02, f * 0.05, 0.]) * 700. + 5.;

            let y = 4000. / x;
            Circle::new()
                .set("cx", x)
                .set("cy", 750. % y + 50.)
                .set("r", 15. * perlin.get([x, y, 0.0]) + 5.)
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
