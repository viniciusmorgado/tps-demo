use godot::{
    classes::{
        ConfigFile, INode, InputEvent, Node, display_server, rendering_server, viewport, window,
    },
    obj::{Base, Gd, NewGd, WithBaseField},
    prelude::{GodotClass, godot_api},
};

enum GIType {
    SDFGI = 0,
    VoxelGi = 1,
    LightMapGi = 2,
}

enum GIQuality {
    DISABLED = 0,
    LOW = 1,
    HIGH = 2,
}

const CONFIG_FILE_PATH: &str = "user://settings.ini";

struct VideoSettings {
    display_mode: window::Mode,
    vsync: display_server::VSyncMode,
    max_fps: u32,
    resolution_scale: f32,
    scale_filter: viewport::Scaling3DMode,
}

struct RenderingSettings {
    taa: bool,
    msaa: viewport::Msaa,
    fxaa: bool,
    shadow_mapping: bool,
    gi_type: GIType,
    gi_quality: GIQuality,
    ssao_quality: rendering_server::EnvironmentSsaoQuality,
    ssil_quality: rendering_server::EnvironmentSsilQuality,
    bloom: bool,
    volumetric_fog: bool,
}

#[derive(GodotClass)]
#[class(base=Node)]
struct Settings {
    base: Base<Node>,
    video_settings: VideoSettings,
    rendering_settings: RenderingSettings,
}

impl Default for RenderingSettings {
    fn default() -> Self {
        Self {
            taa: false,
            msaa: viewport::Msaa::DISABLED,
            fxaa: false,
            shadow_mapping: true,
            gi_type: GIType::VoxelGi,
            gi_quality: GIQuality::LOW,
            ssao_quality: rendering_server::EnvironmentSsaoQuality::LOW,
            ssil_quality: rendering_server::EnvironmentSsilQuality::VERY_LOW,
            bloom: true,
            volumetric_fog: true,
        }
    }
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            display_mode: window::Mode::MAXIMIZED,
            vsync: display_server::VSyncMode::DISABLED,
            max_fps: 0,
            resolution_scale: 1.0,
            scale_filter: viewport::Scaling3DMode::BILINEAR,
        }
    }
}

#[godot_api]
impl INode for Settings {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            video_settings: VideoSettings::default(),
            rendering_settings: RenderingSettings::default(),
        }
    }

    fn ready(&mut self) -> () {
        Self::load_settings()
    }

    fn input(&mut self, input_event: Gd<InputEvent>) -> () {
        Self::toggle_fullscreen(self, &input_event);
    }
}

impl Settings {
    fn save_settings() {
        let mut config = ConfigFile::new_gd();
        config.load(CONFIG_FILE_PATH);
    }

    fn load_settings() {
        let mut config = ConfigFile::new_gd();
        config.save(CONFIG_FILE_PATH);
    }

    fn toggle_fullscreen(&mut self, input_event: &Gd<InputEvent>) {
        if input_event.is_action_pressed("toggle_fullscreen") {
            let mut window = self.base().get_window().unwrap();
            let current_mode = window.get_mode();

            let is_fullscreen = current_mode == window::Mode::EXCLUSIVE_FULLSCREEN
                || current_mode == window::Mode::FULLSCREEN;

            let new_mode = if is_fullscreen {
                window::Mode::WINDOWED
            } else {
                window::Mode::EXCLUSIVE_FULLSCREEN
            };

            window.set_mode(new_mode);
            self.base_mut()
                .get_viewport()
                .unwrap()
                .set_input_as_handled();
        }
    }

    // fn apply_graphics_settings(window: Window, environment: Environment, scene_root: Node) -> () {
    //     // get_window().mode = Settings.config_file.get_value("video", "display_mode")
    //     // DisplayServer.window_set_vsync_mode(Settings.config_file.get_value("video", "vsync"))
    //     // Engine.max_fps = Settings.config_file.get_value("video", "max_fps")
    //     // window.scaling_3d_scale = Settings.config_file.get_value("video", "resolution_scale")
    //     // window.scaling_3d_mode = Settings.config_file.get_value("video", "scale_filter")

    //     // window.use_taa = Settings.config_file.get_value("rendering", "taa")
    //     // window.msaa_3d = Settings.config_file.get_value("rendering", "msaa")
    //     // window.screen_space_aa = Viewport.SCREEN_SPACE_AA_FXAA if Settings.config_file.get_value("rendering", "fxaa") else Viewport.SCREEN_SPACE_AA_DISABLED

    //     // if not Settings.config_file.get_value("rendering", "shadow_mapping"):
    //     // 	# Disable shadows for all lights present during level load,
    //     // 	# reducing the number of draw calls significantly.
    //     // 	# FIXME: In the main menu, shadows aren't enabled again after enabling shadows
    //     // 	# if they were previously disabled. We can't enable shadows on all lights unconditionally,
    //     // 	# as this would negatively affect the level's performance.
    //     // 	scene_root.propagate_call("set", ["shadow_enabled", false])

    //     // if Settings.config_file.get_value("rendering", "ssao_quality") == -1:
    //     // 	environment.ssao_enabled = false
    //     // if Settings.config_file.get_value("rendering", "ssao_quality") == RenderingServer.ENV_SSAO_QUALITY_MEDIUM:
    //     // 	environment.ssao_enabled = true
    //     // 	RenderingServer.environment_set_ssao_quality(RenderingServer.ENV_SSAO_QUALITY_HIGH, false, 0.5, 2, 50, 300)
    //     // else:
    //     // 	environment.ssao_enabled = true
    //     // 	RenderingServer.environment_set_ssao_quality(RenderingServer.ENV_SSAO_QUALITY_MEDIUM, true, 0.5, 2, 50, 300)

    //     // if Settings.config_file.get_value("rendering", "ssil_quality") == -1:
    //     // 	environment.ssil_enabled = false
    //     // elif Settings.config_file.get_value("rendering", "ssil_quality") == RenderingServer.ENV_SSIL_QUALITY_MEDIUM:
    //     // 	environment.ssil_enabled = true
    //     // 	RenderingServer.environment_set_ssil_quality(RenderingServer.ENV_SSIL_QUALITY_MEDIUM, false, 0.5, 2, 50, 300)
    //     // else:
    //     // 	environment.ssil_enabled = true
    //     // 	RenderingServer.environment_set_ssil_quality(RenderingServer.ENV_SSIL_QUALITY_HIGH, true, 0.5, 2, 50, 300)

    //     // environment.glow_enabled = Settings.config_file.get_value("rendering", "bloom")
    //     // environment.volumetric_fog_enabled = Settings.config_file.get_value("rendering", "volumetric_fog")
    // }
}
