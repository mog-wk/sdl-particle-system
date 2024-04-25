use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

#[derive(Debug, Clone)]
enum ParticleShape {
    Rect,
    Circle,
    Star,
}

#[derive(Debug, Clone)]
pub struct Particle {
    x: i32,
    y: i32,

    xvel: i32,
    yvel: i32,

    lifetime: u8,

    color: Color,
    shape: ParticleShape,

    forces: Vec<Force>,
}

impl Particle {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            ..Self::default()
        }
    }
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        match self.shape {
            ParticleShape::Rect => {
                let _ = canvas.fill_rect(Rect::new(self.x, self.y, 2, 2));
            }
            _ => (),
        }
    }

    pub fn update(&mut self) {
        for force in self.forces.iter() {
            self.xvel += force.x;
            self.yvel += force.y;
        }

        self.x += self.xvel;
        self.y += self.yvel;
        self.xvel = 0;
        self.yvel = 0;
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            x: 300,
            y: 200,

            xvel: 0,
            yvel: 0,

            lifetime: 100,

            color: Color::WHITE,
            shape: ParticleShape::Rect,

            forces: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn add(&mut self, p: Particle) {
        self.particles.push(p.to_owned());
    }

    pub fn add_force(&mut self, force: Force) {
        for particle in self.particles.iter_mut() {
            particle.forces.push(force.clone());
        }
    }

    pub fn update(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.update();
        }
    }
}

impl<'a> Default for ParticleSystem {
    fn default() -> Self {
        let particles = Vec::new();
        Self { particles }
    }
}

#[derive(Clone, Debug)]
pub struct Force {
    x: i32,
    y: i32,
    lifetime: Option<u32>,
}
impl Force {
    //pub fn new(x: i32, y: i32, lifetime: Option<u32>) -> Force {
    pub fn new(x: i32, y: i32) -> Force {
        Self {
            x,
            y,
            lifetime: None,
        }
    }
}
