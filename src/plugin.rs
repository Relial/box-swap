use bunny_plugin::{
    GameMode, PluginContext, PluginInfo, hook_builder::NoCbHookPoint, hook_cell::HookCell,
};
use mhfz_structs::MhfzStructs;

use crate::{address::Addresses, hooks};

const PLUGIN_NAME: &str = "Box Swap";
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");

pub static STRUCTS: HookCell<MhfzStructs> = HookCell::new();
static HOOKS: HookCell<Vec<NoCbHookPoint>> = HookCell::new();

// Called once when the plugin is loaded
#[unsafe(no_mangle)]
pub extern "C" fn init(context: &PluginContext) -> PluginInfo {
    let addresses = Addresses::new(context.mhfo_info());
    let dll_info = context.mhfo_info();
    let structs = MhfzStructs::new(dll_info.address, dll_info.game_mode == GameMode::HighGrade);
    let addresses_res: Result<(), MhfzStructs> = STRUCTS.set(structs);
    let mut info = PluginInfo::new(PLUGIN_NAME, PLUGIN_VERSION);

    match hooks::init(&addresses) {
        Ok(hooks) => {
            let hooks_res = HOOKS.set(hooks);
            if addresses_res.is_err() || hooks_res.is_err() {
                info = info.init_fail("State/Hooks init failed: HookCell was already initialized");
            }
            info
        }
        Err(e) => {
            let err_message = format!("Hook init error: {e:#}");
            info = info.init_fail(err_message.as_str());
            info
        }
    }
}

// Called when the plugin is manually disabled by the user
pub fn unload() {
    unsafe {
        HOOKS.drop();
        STRUCTS.drop();
    }
}
