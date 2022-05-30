use cgmath::{InnerSpace, MetricSpace, Vector2, Zero};
use cgmath::num_traits::abs;
use rand::{Rng, thread_rng};
use sfml::graphics::{CircleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::system::Vector2f;

pub struct Circle {

    position: Vector2<f64>,
    prev_position : Vector2<f64>,
    mass : f64,
    size : f64,
    force : Vector2<f64>,

}

impl Circle {

    pub(crate) fn update(&mut self, d_t: f64) {

        let new_position = 2. * self.position - self.prev_position + (self.force / self.mass) * (d_t * d_t);
        self.prev_position = self.position;
        self.position = new_position;
        self.force = Vector2::zero();

    }

    pub fn resolve_collision(&mut self, collision_object: &mut Circle) {


        let mut resolution = (collision_object.get_position() - self.position);
        let mut distance = resolution.magnitude2();

        if distance == 0.{
            resolution = Vector2::new(thread_rng().gen_range(-1.0..1.0), thread_rng().gen_range(-1.0..1.0));
            resolution = resolution.normalize();
            distance = 1.;
        }

        if distance < (self.size + collision_object.get_size()).powi(2) {

            distance = distance.sqrt();

            resolution = resolution / distance * ((self.size + collision_object.get_size()) - distance);
            let split = (self.mass + collision_object.get_mass());

            self.set_raw_position(-resolution * (collision_object.get_mass() / split) + self.position);
            collision_object.set_raw_position(resolution * (self.mass / split) + collision_object.get_position());

        }

    }

    pub fn get_resolution_offset(&self, point : Vector2<f64>) -> Vector2<f64> {

        (point - self.position).normalize_to(self.size) - (point - self.position)

    }

    pub fn check_collision(&self, position : Vector2<f64>) -> bool {

        (position - self.position).magnitude() < self.size

    }

    pub fn force(&mut self, force: Vector2<f64>) {

        self.force += force;

    }

    pub(crate) fn set_raw_position(&mut self, position: Vector2<f64>) {
        self.position = position;
    }

    pub fn set_prev_position(&mut self, prev_position: Vector2<f64>) {
        self.prev_position = prev_position;
    }

    pub fn get_position(&self) -> Vector2<f64> {
        self.position
    }

    pub fn get_prev_position(&self) -> Vector2<f64> {
        self.prev_position
    }

    pub fn set_position(&mut self, position: Vector2<f64>) {
        let velocity = self.get_velocity();

        self.position = position;
        self.prev_position = position - velocity;
    }

    pub fn set_velocity(&mut self, velocity: Vector2<f64>) {
        self.prev_position = self.position - velocity;
    }

    pub fn get_velocity(&self) -> Vector2<f64> {
        self.position - self.prev_position
    }

    pub fn set_force(&mut self, force: Vector2<f64>) {
        self.force = force;
    }
    pub fn get_force(&self) -> Vector2<f64> {
        self.force
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_size(&self) -> f64 {
        self.size
    }

    pub fn new(position : Vector2<f64>, radius : f64, mass: f64) -> Circle {

        Circle {
            position,
            prev_position: position,
            mass,
            size: radius,
            force: Vector2::new(0., 0.),
        }

    }

    pub fn connect(&mut self, other: &mut Circle, distance: f64) {

        let mut offset = other.get_position() - self.get_position();
        let magnitude = offset.magnitude();
        if magnitude == 0. {
            return;
        }
        offset = offset.normalize_to((distance - magnitude) / 2.);
        self.position = -offset + self.get_position();
        other.set_raw_position(offset + other.get_position());

    }

}

impl Clone for Circle {
    fn clone(&self) -> Self {
        Circle {
            position: self.position.clone(),
            prev_position: self.prev_position.clone(),
            mass: self.mass,
            size: self.size,
            force: self.force.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.position = source.position.clone();
        self.prev_position = source.prev_position.clone();
        self.mass = source.mass;
        self.size = source.size;
        self.force = source.force.clone();
    }
}