use ::window::configuration::WindowConfiguration;
use config::configuration;

pub struct ConfigBridge;

impl WindowConfiguration for ConfigBridge {
    fn use_ime(&self) -> bool {
        configuration().use_ime
    }

    fn use_dead_keys(&self) -> bool {
        configuration().use_dead_keys
    }

    fn enable_wayland(&self) -> bool {
        configuration().enable_wayland
    }

    fn prefer_egl(&self) -> bool {
        configuration().prefer_egl
    }
}
