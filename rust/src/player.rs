use godot::engine::{CharacterBody3D, ICharacterBody3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    #[export]
    speed: i32,
    fall_acceleration: i32,
    target_velocity: Vector3,
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            speed: 14,
            fall_acceleration: 75,
            target_velocity: Vector3::ZERO,
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let mut direction = Vector3::ZERO;

        //check input
        let input = Input::singleton();
        if input.is_action_pressed("move_right".into())  {direction.x += 1.}
        if input.is_action_pressed("move_left".into())  {direction.x -= 1.}
        if input.is_action_pressed("move_back".into())  {direction.z += 1.}
        if input.is_action_pressed("move_forward".into())  {direction.z -= 1.}
        direction = direction.normalized();

        //ground velocity
        self.target_velocity.x = direction.x * self.speed as f32;
        self.target_velocity.z = direction.z * self.speed as f32;

        //TODO: vertical velocity

        //apply velocity
        let target_velocity = self.target_velocity;
        //godot_print!("{target_velocity}");
        let mut base = self.base_mut();
        base.set_velocity(target_velocity);
        base.move_and_slide();
    }
}
