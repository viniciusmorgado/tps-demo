use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct EntityTest {
    base: Base<Node>,
}

#[godot_api]
impl INode for EntityTest {
    fn init(base: Base<Node>) -> Self {
        godot_print!("EntityTest initialized!");
        Self { base }
    }
}
