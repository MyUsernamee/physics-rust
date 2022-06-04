use std::borrow::BorrowMut;
use std::time::{Duration, Instant};
use cgmath::{InnerSpace, MetricSpace, Vector2, Zero};
use rand::{Rng, thread_rng};
use sfml::graphics::{CircleShape, Color, Font, Rect, RectangleShape, RenderTarget, RenderWindow, Shape, Text, TextStyle, Transformable};
use sfml::SfBox;
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, Event, Style, VideoMode};
use crate::physics_object::*;
use crate::circle::*;
use std::thread::*;
use sfml::window::mouse::Button;
use slider::Slider;

mod physics_object;
mod circle;
mod slider;

static time_steps: i32 = 8;

fn main() {

    let font: SfBox<Font> = Font::from_file("/usr/share/fonts/truetype/freefont/FreeMono.ttf").unwrap();

    let mut width : f64 = 640.;
    let mut height: f64 = 640.;

    let mut window = RenderWindow::new((640, 640), "Physics", Style::default(), &Default::default());

    let mut physics_world = PhysicsWorld::new(width as u32, height as u32);

    for index in 0..20000 {

        physics_world.push_object(
            Circle::new(Vector2::new(thread_rng().gen_range(0.0..width), thread_rng().gen_range(0.0..height)),
                        1., 1.), CircleShape::new(1., 3));
        physics_world.get_object_mut(index).unwrap().set_velocity(Vector2::new(rand::thread_rng().gen_range(-1.0..1.), 0.));

    }

    let mut right_click_held = false;
    let mut middle_click_held = false;
    let mut left_click_held = false;
    let mut mouse_pos = (0., 0.);
    let mut prev_mouse_pos = (0., 0.);
    let mut particle_grabbed_index = 0;

    let width_clone = width.clone();
    let height_clone = height.clone();

    physics_world.set_update_predicate(Box::new(|object: &mut Circle| {
        object.force(Vector2::new(0., 9.8 * 10. * object.get_mass()));

        let clamped_position = Vector2::new(
            object.get_position().x.clamp(0. + object.get_size(), 640. - object.get_size()),
            object.get_position().y.clamp(0. + object.get_size(), 640. - object.get_size()));
        object.set_raw_position(clamped_position);

    }));

    physics_world.set_draw_predicate(Box::new(|object: &Circle, shape: &mut CircleShape| {
        shape.set_position(Vector2f::new(object.get_position().x as f32, object.get_position().y as f32));
        shape.set_fill_color(Color::rgb((object.get_velocity().x * 255. * 8.).abs().min(255.) as u8,
                                        (object.get_velocity().y * 255. * 8.).abs().min(255.) as u8,
                                        0));
    }));

    loop {
        window.clear(Color::BLACK);

        let t = Instant::now();

        for i in 0..time_steps {

            physics_world.update((1. / 60. / (time_steps as f64)));

            if left_click_held {

                let mut object = &mut physics_world.get_object_mut(particle_grabbed_index).unwrap();

                object.set_raw_position(Vector2::new(mouse_pos.0, mouse_pos.1));
                object.set_prev_position(Vector2::new(prev_mouse_pos.0, prev_mouse_pos.1));


            }

        }

        physics_world.draw(&mut window);

        'event_loop : loop {
            match window.poll_event() {
                Some(Event::Closed) => {

                    return;

                }
                Some(Event::MouseButtonPressed{ button, x, y}) => {

                    match button {

                        Button::LEFT => {

                            left_click_held = true;
                            for object_index in 0..physics_world.get_objects().len() {

                                let object = physics_world.get_object(object_index).unwrap();

                                let mouse_pos_vector = Vector2::new(mouse_pos.0, mouse_pos.1);
                                let offset = mouse_pos_vector - object.get_position();

                                if offset.magnitude() < object.get_size() * 4.0 {

                                    particle_grabbed_index = object_index;
                                    break 'event_loop;
                                }

                            }

                        }
                        Button::RIGHT => {

                            right_click_held = true;

                        }
                        Button::MIDDLE => {

                            middle_click_held = true;

                        }
                        _ => {

                        }

                    }

                }
                Some(Event::MouseButtonReleased{ button, x, y}) => {
                    match button {
                        Button::RIGHT => {
                            right_click_held = false;
                        }
                        Button::MIDDLE => {
                            middle_click_held = false;
                        }
                        Button::LEFT => {
                            left_click_held = false;
                        }
                        _ => {

                        }
                    }
                }
                Some(Event::MouseMoved{ x, y}) => {
                    prev_mouse_pos = mouse_pos;
                    mouse_pos = (x as f64, y as f64);
                }
                None => {break 'event_loop;}
                _ => {}

            }
        }

        if right_click_held {

            physics_world.push_object(
                Circle::new(Vector2::new(mouse_pos.0, mouse_pos.1),
                            4., 1.), CircleShape::new(4.0, 16));

        }

        std::thread::sleep(Duration::from_secs_f64((1. / 60. - t.elapsed().as_secs_f64()).max(0.)));

        let fps = 1. / t.elapsed().as_secs_f64();

        let mut fps_counter = Text::new(&String::from(format!("FPS: {}\nObject Count: {}", fps, physics_world.get_objects().len())), &font, 12);
        fps_counter.set_position(Vector2f::new(10., 10.));
        fps_counter.set_outline_color(Color::BLACK);
        fps_counter.set_outline_thickness(1.);

        if fps < 30. {
            fps_counter.set_fill_color(Color::RED);
        }
        else if fps <  55. {

            fps_counter.set_fill_color(Color::YELLOW);

        }

        window.draw(&fps_counter);
        window.display();

    }

}
