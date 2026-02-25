use graphics::{ellipse::{centered, circle}};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};

use crate::{gfx::gfxutil, object::{self, Object}};

pub struct App {
    pub(crate) gl: GlGraphics, // OpenGL drawing backend.
    ball: Object,
    ground: Object,
}

impl App {

    pub fn new(opengl: OpenGL) -> Self {
        App {
            gl: GlGraphics::new(opengl),
            ball: object::Object::new(5.0,5.0),
            ground: object::Object::new(400.0,10.0),

        }
    }

    pub fn init(&mut self) {
        self.ground.set_color(255, 0, 0);

    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let circle = circle(0.0, 0.0, self.ball.get_size().0);
        let rect = centered([0.0, -200.0, self.ground.get_size().0 / 2.0, self.ground.get_size().1 / 2.0]);

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0); // basically point (0,0)

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear([0.297777778, 0.368888889, 0.475555556, 1.0], gl); // some cool blue color

            let ball_transform = gfxutil::transform::translate(&c, (x,y), (self.ball.get_position().0, self.ball.get_position().1));
            let ground_transform = gfxutil::transform::non_moving(&c, (x,y));


            circle_arc(self.ball.get_color(), self.ball.get_size().0, 0.0, 2.0 * std::f64::consts::PI, circle, ball_transform, gl);
            rectangle(self.ground.get_color(), rect, ground_transform, gl);
            
        });
    }


    pub fn update(&mut self, args: &UpdateArgs) {
        let x = self.ball.get_position().0 + 50.0 * args.dt;
        let y = self.ball.get_position().1 + 50.0 * args.dt;


        self.ball.set_position(x, y);
        println!("ball: {}", self.ball.to_string())
    }
}