use std::cmp::Ordering;
use std::ops::DerefMut;
use std::sync::mpsc;
use cgmath::{MetricSpace, Vector2};
use sfml::graphics::{CircleShape, Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::Event::Resized;
use sfml::window::Window;
use crate::{Circle, Zero};
use std::thread::*;
use cgmath::num_traits::FloatConst;
use sfml::system::Vector2f;

const GRID_SIZE: usize = 320;

pub struct PhysicsWorld {

    objects : Vec<Circle>,
    size : (u32, u32),
    grid : Vec<Vec<usize>>,
    update_predicate: Box<dyn Fn(&mut Circle)>,
    draw_predicate: Box<dyn Fn(&Circle, &mut CircleShape)>,
    circle_shapes: Vec<CircleShape<'static>>,

}

impl PhysicsWorld {

    pub(crate) fn new(width: u32, height : u32) -> PhysicsWorld {


        let mut new_world = PhysicsWorld {
            objects: Vec::new(),
            size: (width, height),
            grid: Vec::new(),
            circle_shapes: Vec::new(),
            update_predicate: Box::new(|_| {}),
            draw_predicate: Box::new(|_, _| {}),
        };

        for _ in 0..GRID_SIZE * GRID_SIZE {
            new_world.grid.push(Vec::new());
        }

        return new_world;

    }

    pub fn set_update_predicate(&mut self, predicate: Box<dyn Fn(&mut Circle)>) {
        self.update_predicate = predicate;
    }

    pub fn set_draw_predicate(&mut self, predicate: Box<dyn Fn(&Circle, &mut CircleShape)>) {
        self.draw_predicate = predicate;
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

            for grid_x_offset in 0..3 {
                for grid_y_offset in 0..3 {

                    let offset_index : usize = (grid_index as i32 + (grid_x_offset as i32 - 1) + (grid_y_offset as i32 - 1) * GRID_SIZE as i32) as usize;

                    if offset_index >= GRID_SIZE * GRID_SIZE {
                        continue;
                    }

                    for other_object_index_index in 0..self.grid[offset_index].len() {
                        let other_object_index = *self.grid[offset_index].get(other_object_index_index).unwrap();

                        if other_object_index == object_index {
                            continue;
                        }

                        let other_object;
                        let object_to_check;

                        if other_object_index > object_index {
                            let (head, tail) = self.objects.split_at_mut(other_object_index);
                            other_object = &mut tail[0];
                            object_to_check = &mut head[object_index];
                        } else {
                            let (head, tail) = self.objects.split_at_mut(object_index);
                            other_object = &mut head[other_object_index];
                            object_to_check = &mut tail[0];
                        }

                        object_to_check.resolve_collision(other_object);

                    }
                }
            }

        }


        //self.objects.sort_by(|a, b| {
        //    if a.get_position().y < b.get_position().y {
        //        return Ordering::Less;
        //    } else if a.get_position().y > b.get_position().y {
        //        return Ordering::Greater;
        //    } else {
        //        if a.get_position().x < b.get_position().x {
        //            return Ordering::Less;
        //        } else if a.get_position().x > b.get_position().x {
        //            return Ordering::Greater;
        //        } else {
        //            return Ordering::Equal;
        //        }
        //    }
        //});

        for index in 0..GRID_SIZE * GRID_SIZE {
            self.grid[index].clear();
        }

        for object_index in 0..self.objects.len() as usize {

            let object = &mut self.objects[object_index];
            object.update(d_t);
            (self.update_predicate)(object);
            let grid_index = self.convert_to_grid_index(self.objects[object_index].get_position().x, self.objects[object_index].get_position().y);
            self.grid[grid_index].push(object_index);
        }

    }

    pub fn draw(&mut self, window : &mut RenderWindow){

        for object_index in 0..self.circle_shapes.len() {
            (self.draw_predicate)(&self.objects[object_index], &mut self.circle_shapes[object_index]);
            window.draw(&self.circle_shapes[object_index]);
        }

        // Visualize the grid
        /*for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let mut rect = RectangleShape::new();
                rect.set_size(Vector2f::new((self.size.0 as f32 / GRID_SIZE as f32), (self.size.1 as f32 / GRID_SIZE as f32)));
                rect.set_position(Vector2f::new(x as f32 * (self.size.0 as f32 / GRID_SIZE as f32), y as f32 * (self.size.1 as f32 / GRID_SIZE as f32)));
                let grid_particle_count = self.grid[x + y * GRID_SIZE].len();
                rect.set_fill_color(Color::rgba((x as f64 / GRID_SIZE as f64 * 255.) as u8, (y as f64 / GRID_SIZE as f64 * 255.) as u8, (grid_particle_count as f64 * 2  as f64 * 255.) as u8, 50));
                rect.set_outline_thickness(1.0);
                rect.set_outline_color(Color::rgb(50, 50, 50));
                window.draw(&rect);
            }
        }*/

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

        let grid_index = self.convert_to_grid_index(circle.get_position().x, circle.get_position().y);
        self.grid[grid_index].push(self.objects.len());
        self.objects.push(circle);
        self.circle_shapes.push(circle_shape);

    }

}

