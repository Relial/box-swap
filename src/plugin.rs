use bunny_plugin::{PluginContext, PluginInfo, hook_builder::NoCbHookPoint, hook_cell::HookCell};
use tracing::error;

use crate::{address::Addresses, hooks};

const PLUGIN_NAME: &str = "Box Swap";
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");

pub static ADDRESSES: HookCell<Addresses> = HookCell::new();
static HOOKS: HookCell<Vec<NoCbHookPoint>> = HookCell::new();

// Called once when the plugin is loaded
#[unsafe(no_mangle)]
pub extern "C" fn init(context: PluginContext) -> PluginInfo {
    tracing_subscriber::fmt()
        .without_time()
        .with_ansi(false)
        .with_max_level(context.log_level())
        .init();

    let addresses = Addresses::new(context.mhfo_info());
    let addresses_res = ADDRESSES.set(addresses);
    let mut info = PluginInfo::new(PLUGIN_NAME, PLUGIN_VERSION);

    match hooks::init(&addresses) {
        Ok(hooks) => {
            let hooks_res = HOOKS.set(hooks);
            if addresses_res.is_err() || hooks_res.is_err() {
                info = info
                    .with_init_fail("State/Hooks init failed: HookCell was already initialized");
            }
            info
        }
        Err(e) => {
            let err_message = format!("Hook init error: {e:#}");
            info = info.with_init_fail(err_message.as_str());
            error!(err_message);
            info
        }
    }
}

// Called every frame when the plugin's dropdown in the manager window is open
#[unsafe(no_mangle)]
pub extern "C" fn menu(_: &mut usize) {}

// Called every frame
#[unsafe(no_mangle)]
pub extern "C" fn ui(_: &mut usize) {}

// Called once per user defined autosave interval, and when the plugin is manually disabled by the user or the game is closed
#[unsafe(no_mangle)]
pub extern "C" fn save() {}

// Called when the plugin is manually disabled by the user
pub fn unload() {
    unsafe {
        HOOKS.drop();
        ADDRESSES.drop();
    }
}
