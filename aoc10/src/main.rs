use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;
use serde_scan::scan;

#[derive(Debug, Copy, Clone)]
struct Vector(i32, i32);
#[derive(Debug)]
struct Particle {
    position: Position,
    velocity: Velocity,
}
type Position = Vector;
type Velocity = Vector;
type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl Particle {
    fn update(&mut self) {
        self.position += self.velocity;
    }
}

fn main() -> Result<()> {
    let mut particles = std::fs::read_to_string("input")?
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<Particle>>>()?;
    for i in 0..10303 {
        particles.iter_mut().for_each(|p| p.update());
        // println!("{:?}", particles[0]);
        // if i % 1000 == 0 {
        //     plot(&particles, format!("plot_{}.svg", i).as_str());
        // }
    }
    plot(&particles, format!("plot1.svg").as_str());
    particles.iter_mut().for_each(|p| p.update());
    plot(&particles, format!("plot2.svg").as_str());
    particles.iter_mut().for_each(|p| p.update());
    plot(&particles, format!("plot3.svg").as_str());
    Ok(())
}

fn plot(particles: &[Particle], file_name: &str) {
    let data: Vec<(f64, f64)> = particles
        .iter()
        .map(|p| (p.position.0 as f64, p.position.1 as f64))
        .collect();
    let scatter: Plot = Plot::new(data).point_style(PointStyle::new().marker(PointMarker::Square));
    let view = ContinuousView::new().add(scatter);
    Page::single(&view).save(file_name).unwrap();
}

fn parse_line(line: &str) -> Result<Particle> {
    let stripped: String = line.split_whitespace().collect();
    let stripped_str = stripped.as_str();
    let (x, y, dx, dy): (i32, i32, i32, i32) =
        scan!("position=<{},{}>velocity=<{},{}>" <- stripped_str)?;
    Ok(Particle {
        position: Vector(x, y),
        velocity: Vector(dx, dy),
    })
}
