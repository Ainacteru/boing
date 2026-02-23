use graphics::ellipse::circle;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};

use crate::object;

pub struct App {
    pub(crate) gl: GlGraphics, // OpenGL drawing backend.
    position: (f64, f64),  // Rotation for the square.
}

impl App {

    pub fn new(opengl: OpenGL) -> Self {
        App {
            gl: GlGraphics::new(opengl),
            position: (0.0, 0.0), // default value
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let ball = object::Object::new(0.0,0.0,10.0,10.0,0,0,0);

        let circle = circle(ball.get_position().0, ball.get_position().1, ball.get_size().0);

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x + self.position.0, y + self.position.1);

            // Draw a box rotating around the middle of the screen.
            circle_arc(RED, ball.get_size().0, 0.0, 2.0*3.14, circle, transform, gl);

            
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.position.0 += 20.0 * args.dt; // pos x
        self.position.1 -= 20.0 * args.dt; // pos y
    }
}