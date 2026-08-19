use anyhow::Result;
use bunny_plugin::hook_builder::{NoCbHookBuilder, NoCbHookPoint};
use ilhook::x86::{HookType, Registers};

use crate::{address::Addresses, plugin::STRUCTS};

unsafe extern "cdecl" fn on_interaction(reg: *mut Registers, _: usize) {
    unsafe {
        let structs = STRUCTS.get_unchecked();
        if (*reg).esi == 0x26
            && let Some(player) = structs.own_player()
            && !player.quest_accepted()
        {
            (*reg).esi = 0x35;
        }
    }
}

fn hook_interaction(addresses: &Addresses) -> Result<NoCbHookPoint> {
    let hook_address = addresses.interaction;
    let builder = NoCbHookBuilder::new(hook_address, HookType::JmpBack(on_interaction));
    let hook_point = unsafe { builder.hook() }?;
    Ok(hook_point)
}

pub fn init(addresses: &Addresses) -> Result<Vec<NoCbHookPoint>> {
    Ok(vec![hook_interaction(addresses)?])
}
