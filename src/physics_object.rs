use std::ops::DerefMut;
use std::sync::mpsc;
use cgmath::Vector2;
use sfml::graphics::{CircleShape, Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::Event::Resized;
use sfml::window::Window;
use crate::Circle;
use std::thread::*;
use cgmath::num_traits::FloatConst;
use sfml::system::Vector2f;
use array_init::array_init;

const GRID_SIZE: usize = 16;

pub struct PhysicsWorld {

    objects : Vec<Circle>,
    size : (u32, u32),
    grid : [Vec<usize>; GRID_SIZE * GRID_SIZE],
    circle_shapes: Vec<CircleShape<'static>>,
    thread_count: usize,

}

impl PhysicsWorld {

    pub(crate) fn new(width: u32, height : u32 ,thread_count: usize) -> PhysicsWorld {


        let new_world = PhysicsWorld {
            objects: Vec::new(),
            size: (width, height),
            grid: array_init::array_init(|_| Vec::new()),
            circle_shapes: Vec::new(),
            thread_count,
        };

        return new_world;

    }

    fn convert_to_grid_index(&self, x: f64, y: f64) -> usize {

        let x_index = (x / self.size.0 as f64 * GRID_SIZE as f64).min((GRID_SIZE - 1) as f64) as usize;
        let y_index = (y / self.size.1 as f64 * GRID_SIZE as f64).min((GRID_SIZE - 1) as f64) as usize;

        return x_index + y_index * GRID_SIZE;

    }

    pub fn update (&mut self, d_t: f64) {

        for object_index in 0..self.objects.len() as usize {

            let object = &mut self.objects[object_index];
            let object_position = object.get_position();
            let grid_index = self.convert_to_grid_index(object_position.x, object_position.y);

            for grid_x_offset in 0..2 {

                for grid_y_offset in 0..2 {

                    let offset_index = grid_index + (grid_x_offset - 1) + (grid_y_offset - 1) * GRID_SIZE;

                    if offset_index >= GRID_SIZE * GRID_SIZE {
                        continue;
                    }

                    for other_object_index in self.grid[offset_index].iter() {
                        if other_object_index == &object_index {
                            continue;
                        }

                        let other_object;
                        let object_to_check;

                        if other_object_index > &object_index {
                            let (head, tail) = self.objects.split_at_mut(*other_object_index);
                            other_object = &mut tail[0];
                            object_to_check = &mut head[object_index];
                        }
                        else {
                            let (head, tail) = self.objects.split_at_mut(object_index);
                            other_object = &mut head[*other_object_index];
                            object_to_check = &mut tail[0];
                        }

                        object_to_check.resolve_collision(other_object);
                    }

                }

            }

        }

        for i in 0..GRID_SIZE * GRID_SIZE {
            self.grid[i].clear();
        }

        for index in 0..self.objects.len() {

            let mut object = &mut self.objects[index];

            object.update(d_t);

            let object_position = object.get_position();

            let grid_index = self.convert_to_grid_index(object_position.x, object_position.y);

            self.grid[grid_index].push(index);

        }

        //for object_index in 0..(self.objects.len() as f64 / 2.).ceil() as usize {
//
        //    for collision_object_index in object_index + 1..self.objects.len() {
//
        //        let (head, tail) = self.objects.split_at_mut(collision_object_index);
        //        let object = &mut head[object_index];
        //        let collision_object = &mut tail[0];
//
        //        //object.resolve_collision(collision_object);
//
        //    }
        //}

    }

    //pub fn update(&mut self, d_t : f64) {
//
    //    let mut threads = Vec::new();
//
    //    let (tx, rx) = mpsc::channel();
    //    for thread_index in 0..self.thread_count {
//
    //        let start_index = thread_index * (self.objects.len() / self.thread_count);
    //        let end_index = if thread_index == self.thread_count - 1 {
    //            self.objects.len()
    //        } else {
    //            (thread_index + 1) * (self.objects.len() / self.thread_count)
    //        };
//
    //        // Color the balls based on thread index
    //        //for (index, circle) in self.objects.iter_mut().enumerate() {
    //        //    if index >= start_index && index < end_index {
////
    //        //        let red = f32::sin((thread_index as f32 / self.thread_count as f32) * f32::PI()) * 255.0;
    //        //        // Green is the same as red but with a different phase
    //        //        let green = f32::sin((thread_index as f32 / self.thread_count as f32) * f32::PI() + f32::PI() * 2. * (1. / 3.)) * 255.0;
    //        //        // Blue is the same as red but with a different phase
    //        //        let blue = f32::sin((thread_index as f32 / self.thread_count as f32) * f32::PI() + f32::PI() * 2. * (2. / 3.)) * 255.0;
    //        //        self.circle_shapes[index].set_fill_color(sfml::graphics::Color::rgb(red as u8, green as u8, blue as u8));
    //        //    }
    //        //}
//
    //        let mut thread_mut_objects = self.objects[start_index..end_index].to_vec();
    //        let mut old_objects = self.objects.clone();
//
    //        let dual_object_predicate = self.dual_object_predicate;
    //        let object_predicate = self.object_predicate;
//
    //        let tx_clone = tx.clone();
    //        let handler = spawn(move || {
//
    //            for object_index in 0..old_objects.len() {
    //                for collision_object_index in 0..thread_mut_objects.len() {
    //                    if (start_index + collision_object_index) == object_index {
    //                        continue;
    //                    }
//
    //                    let object = &old_objects[object_index];
    //                    let collision_object = &mut thread_mut_objects[collision_object_index];
//
    //                    collision_object.resolve_collision(object);
//
    //                    dual_object_predicate(collision_object, object);
    //                }
    //            }
//
//
    //            let mut final_objects: Vec<(usize, Circle)> = Vec::new();
//
    //            for object_index in 0..thread_mut_objects.len() {
    //                let mut object = &mut thread_mut_objects[object_index];
    //                object.update(d_t);
    //                object_predicate(&mut object);
    //                final_objects.push((object_index + start_index, thread_mut_objects[object_index].clone()));
    //            }
//
    //            tx_clone.send(final_objects).unwrap();
//
    //        });
//
    //        threads.push(handler);
//
    //    }
//
    //    /*for object_index in 0..self.objects.len() {
//
    //        &mut self.objects[object_index].update(d_t);
    //        (self.object_predicate)(&mut self.objects[object_index]);
//
    //        for collision_object_index in 0..self.objects.len() {
//
    //            if collision_object_index == object_index {
    //                continue;
    //            }
//
    //            let object =  &old_objects[object_index];
    //            let collision_object = &mut self.objects[collision_object_index];
//
    //            collision_object.resolve_collision(object);
//
    //            (self.dual_object_predicate)(collision_object, object);
//
    //        }
//
    //    }*/
//
    //    let mut receive_count = 0;
//
    //    while receive_count < self.thread_count {
    //        match rx.recv() {
    //            Ok(mut objects) => {
    //                for mut object in objects {
    //                    self.objects[object.0] = object.1;
    //                }
    //                receive_count += 1;
    //            }
    //            _ => {
    //                panic!("Error receiving objects");
    //            }
    //        }
    //    }
//
    //}

    pub fn draw(&mut self, window : &mut RenderWindow){

        for object_index in 0..self.circle_shapes.len() {
            self.circle_shapes[object_index].set_position(Vector2f::new((self.objects[object_index].get_position().x - self.objects[object_index].get_size()) as f32, (self.objects[object_index].get_position().y - self.objects[object_index].get_size()) as f32));
            window.draw(&self.circle_shapes[object_index]);
        }

        // Visualize the grid
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let mut rect = RectangleShape::new();
                rect.set_size(Vector2f::new((self.size.0 as f32 / GRID_SIZE as f32), (self.size.1 as f32 / GRID_SIZE as f32)));
                rect.set_position(Vector2f::new(x as f32 * (self.size.0 as f32 / GRID_SIZE as f32), y as f32 * (self.size.1 as f32 / GRID_SIZE as f32)));
                let grid_particle_count = self.grid[x + y * GRID_SIZE].len();
                rect.set_fill_color(Color::rgba((x as f64 / GRID_SIZE as f64 * 255.) as u8, (y as f64 / GRID_SIZE as f64 * 255.) as u8, (grid_particle_count as f64 / self.objects.len() as f64 * 255.) as u8, 50));
                rect.set_outline_thickness(1.0);
                rect.set_outline_color(Color::rgb(50, 50, 50));
                window.draw(&rect);
            }
        }

    }

    pub fn get_objects(&self) -> &Vec<Circle> {

        return &self.objects;

    }

    pub fn get_objects_mut(&mut self) -> &mut Vec<Circle> {

        &mut self.objects

    }

    pub fn get_object(&self, index : usize) -> Option<&Circle> {

        self.objects.get(index)

    }

    pub fn get_object_mut(&mut self, index : usize) -> Option<&mut Circle> {

        self.objects.get_mut(index)

    }

    pub fn push_object(&mut self, circle: Circle, circle_shape: CircleShape<'static>) {

        self.objects.push(circle);
        self.circle_shapes.push(circle_shape);

    }

}

