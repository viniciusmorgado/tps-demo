use godot::prelude::*;

mod entity_test;

struct TpsDemo;

#[gdextension]
unsafe impl ExtensionLibrary for TpsDemo {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("TPS Demo Core (Rust) carregado com sucesso!");
            godot_print!("GDExtension inicializada - Level: {:?}", level);
        }
    }
}
