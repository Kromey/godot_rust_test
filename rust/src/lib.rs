use godot::engine::{ISprite3D, Sprite3D};
use godot::prelude::*;
use godot::classes::{INode3D, Node3D};

mod gen;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node3D)]
struct Dancer {
    direction_z: f64,
    direction_x: f64,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Dancer {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            direction_z: -1.0,
            direction_x: -1.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut position = self.base().get_position();

        position.z += (self.direction_z * delta) as f32;
        position.x += (self.direction_x * delta / 2.0) as f32;

        self.base_mut().set_position(position);

        if position.z > 1.0 {
            self.direction_z = -1.0;
        } else if position.z < -1.0 {
            self.direction_z = 1.0;
        }

        if position.x > 1.0 {
            self.direction_x = -1.0;
        } else if position.x < -1.0 {
            self.direction_x = 1.0;
        }
    }
}

#[derive(GodotClass)]
#[class(base=Sprite3D)]
struct MagicSprite {
    base: Base<Sprite3D>,
}

#[godot_api]
impl ISprite3D for MagicSprite {
    fn init(base: Base<Sprite3D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let img_texture = gen::generate_sprite();
        self.base_mut().set_texture(img_texture.upcast());
    }
}
