extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use rand::distributions::{IndependentSample, Range};

static WIDTH: u32 = 640;
static HEIGHT: u32 = 480;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    world:Vec<Cell>

}

pub struct Cell {
    x:f64,
    y:f64,
    color: [f32; 4]
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let ref world = self.world;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);
            
            for cell in world {
                // println!("{:?}", (i as u32%WIDTH) as  f64);
                rectangle(cell.color,
                    [cell.x * 5.0, cell.y * 5.0, 5.0, 5.0],
                    c.transform, gl);
            }

            // rectangle([1.0, 0.0, 0.0, 1.0], // red
            //           [0.0, 0.0, 100.0, 100.0],
            //           c.transform, gl);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn pick(a: i32, b: i32) -> i32 {
    let between = Range::new(a, b);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

enum TYPE {
    NOTHING,
    PLANT,
    ANIMAL
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    println!("OHH YEAS");


    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [WIDTH, HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut world:Vec<Cell> = Vec::new();

    for x in 0..(WIDTH/5){
        for y in 0..(HEIGHT/5){
            // let type = pick(0, 200) as f32;
            let typee = match pick(0, 200) {
                0 ... 150 => TYPE::NOTHING,
                151 ... 198 => TYPE::PLANT,
                199 ... 200 => TYPE::ANIMAL,
                _ => TYPE::NOTHING
            };

            let color = match typee {
                TYPE::NOTHING =>  [0.2, 0.3, 0.2, 1.0],
                TYPE::PLANT =>  [0.0, 1.0, 0.0, 1.0],
                TYPE::ANIMAL =>  [1.0, 0.3, 0.0, 1.0]
            };

            let num = pick(0, 200) as f32;
            world.push(Cell{color: color, x: x as f64, y: y as f64});
        }
    }

    // world.resize((WIDTH * HEIGHT)as usize, [0.0, 0.0, 1.0, 1.0]);
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        world
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}