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
use rand::{thread_rng, Rng};

static WIDTH: u32 = 640;
static HEIGHT: u32 = 480;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    cells:Vec<Cell>

}

pub struct Cell {
    x:f64,
    y:f64,
    dna: CellDNA
}


impl Cell {
    fn new(x:f64, y:f64, color: [f32; 4]) -> Cell {
        Cell {
            x:x, y:y, dna: CellDNA::new(color)
        }
    }
}

pub struct CellDNA {
    energy:      f32,
    color:       [f32; 4],
    speed:       f32,
    health:      f32,
    photosynth:  bool,
    eats_food:   bool,
    size:        f32,
    energyUsage: f32
}

impl CellDNA {
    fn new(color: [f32; 4]) -> CellDNA {
        CellDNA {
            energy:      pickf32(0, 100),
            color:       color,
            speed:       pickf32(0, 100),
            health:      pickf32(0, 100),
            photosynth:  pick(0, 100) < 10,
            eats_food:   pick(0, 100) > 90,
            size:        pickf32(0, 100),
            energyUsage: pickf32(0, 100)
        }
    }
}

fn pick(a: i32, b: i32) -> i32 {
    let between = Range::new(a, b);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

fn pickf32(a: i32, b: i32) -> f32 {
    pick(a  * 100, b * 100)as f32  / 100.0
}

fn randColor() -> [f32; 4] {
    [pickf32(0, 1), pickf32(0, 1), pickf32(0, 1), 1.0]
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        {
            let cell = rand::thread_rng().choose_mut(&mut self.cells).unwrap();
            cell.dna.color = randColor();
        }

        let ref cells = self.cells;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);
            
            for cell in cells {
                // println!("{:?}", (i as u32%WIDTH) as  f64);
                rectangle(cell.dna.color,
                    [cell.x * 5.0, cell.y * 5.0, 5.0, 5.0],
                    c.transform, gl);
            }

            // self.cells[0].dna.color = randColor();

            // let cell = rand::thread_rng().choose_mut(&self.cells);
            // cell.dna.color = randColor();
            // let randColor = rand::thread_rng().choose(&cell.dna.color));
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


enum TYPE {
    NOTHING = 150,
    PLANT = 198,
    ANIMAL = 200
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

    let mut cells:Vec<Cell> = Vec::new();

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
            cells.push(Cell::new(x as f64, y as f64, color));
        }
    }

    // cells.resize((WIDTH * HEIGHT)as usize, [0.0, 0.0, 1.0, 1.0]);
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        cells
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