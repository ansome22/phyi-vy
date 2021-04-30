
mod time_accumulator;

use glamifiedphysics::scene::{BodyHandle, PhysicsScene};
use time_accumulator::TimeAccumulator;

struct GlamifiedPhysics;

fn physics_update_system(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut accum: ResMut<TimeAccumulator>,
    mut scene: ResMut<PhysicsScene>,
) {
    let delta = time.delta();
    accum.update(delta);

    let num_steps = if scene.paused {
        0
    } else {
        accum.num_steps()
    };

    let step_secs = accum.step_secs();
    for _ in 0..num_steps {
        // the game physics weekend application is doing 2 sub steps
        for _ in 0..2 {
            scene.update(step_secs * 0.5);
        }
    }
}

fn copy_transforms_system(
    physics_scene: Res<PhysicsScene>,
    mut query: Query<(&BodyHandle, &mut Transform)>,
) {
    for (body_handle, mut transform) in query.iter_mut() {
        let body = physics_scene.get_body(body_handle);
        transform.translation = body.position;
        transform.rotation = body.orientation;
    }
}

impl Plugin for MyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(TimeAccumulator::new())
            .add_system(physics_update_system.system())
            .add_system(copy_transforms_system.system())
    }
}

fn main() {
    App::build()
        .add_plugin(GlamifiedPhysics)
        .run();
}