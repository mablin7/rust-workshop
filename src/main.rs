extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rapier2d::prelude::*;

const ROBOT_RADIUS: f64 = 0.2;
const BALL_RADIUS: f64 = 0.045;
const PIXELS_PER_METER: f64 = 100.0;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

fn meters_to_pixels(meters: f32) -> i32 {
    meters as i32 * PIXELS_PER_METER as i32
}

struct Robot {
    body_handle: RigidBodyHandle,
}

struct Ball {
    body_handle: RigidBodyHandle,
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.

    physics_pipeline: PhysicsPipeline,
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),

    robots: Vec<Robot>,
    ball: Ball,
}

fn create_robot(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    x: f32,
    y: f32,
) -> Robot {
    let position = Isometry::new(Vector::new(x, y), 0.0);
    let rb = RigidBodyBuilder::dynamic().position(position).build();
    let handle = rigid_body_set.insert(rb);
    let collider = ColliderBuilder::ball(ROBOT_RADIUS as f32).build();
    collider_set.insert_with_parent(collider, handle, rigid_body_set);
    Robot {
        body_handle: handle,
    }
}

fn create_ball(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    x: f32,
    y: f32,
) -> Ball {
    let position = Isometry::new(Vector::new(x, y), 0.0);
    let rb = RigidBodyBuilder::dynamic().position(position).build();
    let handle = rigid_body_set.insert(rb);
    let collider = ColliderBuilder::ball(BALL_RADIUS as f32).build();
    collider_set.insert_with_parent(collider, handle, rigid_body_set);
    Ball {
        body_handle: handle,
    }
}

// This function creates n robots arranged in a neat row
fn arrange_robots(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    n: usize,
) -> Vec<Robot> {
    let mut robots = Vec::new();
    for i in 0..n {
        let x = 60.0 + i as f32 * 2.0 * (ROBOT_RADIUS + 0.1) as f32 * PIXELS_PER_METER as f32;
        let y = PIXELS_PER_METER as f32 * 2.0 * ROBOT_RADIUS as f32;
        robots.push(create_robot(rigid_body_set, collider_set, x, y));
    }
    robots
}

impl App {
    fn run() {
        let opengl = OpenGL::V3_2;
        let mut window: Window = WindowSettings::new("spinning-square", [500, 500])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let gravity = vector![0.0, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        let ball = create_ball(&mut rigid_body_set, &mut collider_set, 150.0, 250.0);
        let robots = arrange_robots(&mut rigid_body_set, &mut collider_set, 6);

        let mut app = App {
            gl: GlGraphics::new(opengl),
            gravity,
            integration_parameters,
            physics_pipeline,
            rigid_body_set,
            collider_set,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            physics_hooks,
            event_handler,
            ball,
            robots,
        };

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(args) = e.render_args() {
                app.render(&args);
            }

            if let Some(args) = e.update_args() {
                app.update(&args);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const ROBOT_RECT: [f64; 4] = [
            0.0,
            0.0,
            ROBOT_RADIUS * PIXELS_PER_METER * 2.0,
            ROBOT_RADIUS * PIXELS_PER_METER * 2.0,
        ];
        const BALL_RECT: [f64; 4] = [
            0.0,
            0.0,
            BALL_RADIUS * PIXELS_PER_METER * 2.0,
            BALL_RADIUS * PIXELS_PER_METER * 2.0,
        ];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            // Draw robots
            for robot in &self.robots {
                if let Some(rb) = self.rigid_body_set.get(robot.body_handle) {
                    let pos = rb.position().translation;
                    let omega = rb.position().rotation.angle();
                    let transform = c
                        .transform
                        .trans(pos.x.into(), pos.y.into())
                        .rot_rad(omega.into());
                    ellipse(RED, ROBOT_RECT, transform, gl);
                }
            }

            // Draw ball
            if let Some(rb) = self.rigid_body_set.get(self.ball.body_handle) {
                let pos = rb.position().translation;
                let omega = rb.position().rotation.angle();
                let transform = c
                    .transform
                    .trans(pos.x.into(), pos.y.into())
                    .rot_rad(omega.into());
                ellipse(RED, BALL_RECT, transform, gl);
            }
        });
    }

    fn update(&mut self, _: &UpdateArgs) {
        // Give robots downward velocity
        for robot in &self.robots {
            if let Some(rb) = self.rigid_body_set.get_mut(robot.body_handle) {
                rb.
            }
        }

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &self.physics_hooks,
            &self.event_handler,
        );
    }
}

fn main() {
    // go
    App::run();
}
