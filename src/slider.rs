use cgmath::{MetricSpace, Vector2};
use sfml::graphics::*;
use sfml::system::Vector2f;

struct CircleLine {
    start: Vector2f,
    end: Vector2f,
    radius: f64,
    steps: u32,
    color: sfml::graphics::Color,
}
impl CircleLine {

    pub fn new (start: Vector2f, end: Vector2f, radius: f64, steps: u32, color: sfml::graphics::Color) -> CircleLine {
        CircleLine {
            start,
            end,
            radius,
            steps,
            color,
        }
    }

    pub fn draw(&self, window: &mut sfml::graphics::RenderWindow) {

        for i in 0..self.steps + 1 {

            let position = (self.end - self.start) * (i as f32 / self.steps as f32) + self.start;

            let mut circle = CircleShape::new(self.radius as f32, 16);
            circle.set_position(position);
            circle.set_fill_color(self.color);
            window.draw(&circle);

        }

    }

    pub fn set_color(&mut self, color: sfml::graphics::Color) {
        self.color = color;
    }
    pub fn get_color(&self) -> sfml::graphics::Color {
        self.color
    }
    pub fn set_start(&mut self, start: Vector2f) {
        self.start = start;
    }
    pub fn get_start(&self) -> Vector2f {
        self.start
    }
    pub fn set_end(&mut self, end: Vector2f) {
        self.end = end;
    }
    pub fn get_end(&self) -> Vector2f {
        self.end
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
    pub fn set_steps(&mut self, steps: u32) {
        self.steps = steps;
    }
    pub fn get_steps(&self) -> u32 {
        self.steps
    }

}

pub(crate) struct Slider {

    position: Vector2f,
    value: f64,
    length: f64,
    radius: f64,
    circle_shape: CircleShape<'static>,
    circle_line: CircleLine,
    handle_color: sfml::graphics::Color,
    line_color: sfml::graphics::Color,

}
impl Slider {

    pub fn new(position: Vector2f, length: f64, radius: f64, handle_color: sfml::graphics::Color, line_color: sfml::graphics::Color) -> Slider {
        Slider {
            position,
            value: 0.,
            length,
            radius,
            circle_shape: CircleShape::new(radius as f32, 16),
            circle_line: CircleLine::new(Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0), radius, 16, line_color),
            handle_color,
            line_color,
        }
    }

    pub fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {


        self.circle_line.set_start(Vector2f::new(self.position.x, self.position.y - self.radius as f32));
        self.circle_line.set_end(Vector2f::new(self.position.x + self.length as f32, self.position.y - self.radius as f32));
        self.circle_line.set_color(self.line_color);
        self.circle_line.draw(window);

        self.circle_shape.set_position(Vector2f::new(self.position.x + (self.value * self.length) as f32, self.position.y - self.radius as f32));
        self.circle_shape.set_fill_color(self.handle_color);
        window.draw(&self.circle_shape);

    }

    pub fn update(&mut self, mouse_position : Vector2<f64>, mouse_held : bool) {

        if mouse_held && mouse_position.distance(Vector2::new((self.position.x + (self.value * self.length) as f32) as f64, self.position.y as f64)) < self.length as f64 {
            self.value = ((mouse_position.x - self.position.x as f64) / self.length).min(1.0).max(0.0);
        }

    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
    pub fn set_position(&mut self, position: Vector2f) {
        self.position = position;
    }
    pub fn get_position(&self) -> Vector2f {
        self.position
    }
    pub fn set_length(&mut self, length: f64) {
        self.length = length;
    }
    pub fn get_length(&self) -> f64 {
        self.length
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
    pub fn set_handle_color(&mut self, handle_color: sfml::graphics::Color) {
        self.handle_color = handle_color;
    }
    pub fn get_handle_color(&self) -> sfml::graphics::Color {
        self.handle_color
    }
    pub fn set_line_color(&mut self, line_color: sfml::graphics::Color) {
        self.line_color = line_color;
    }
    pub fn get_line_color(&self) -> sfml::graphics::Color {
        self.line_color
    }

}