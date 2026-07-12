use bunny_plugin::{GameMode, MhfoInfo};
use mhfz_structs::player::{Player, PlayerInfo};

#[derive(Clone, Copy, Debug)]
pub struct Addresses {
    pub interaction: usize,
    player_structs: usize,
    player_info: usize,
}

impl Addresses {
    pub fn new(mhfo_info: MhfoInfo) -> Self {
        let dll = mhfo_info.address;
        match mhfo_info.game_mode {
            GameMode::LowGrade => Self {
                interaction: dll + 0x69b7f7,
                player_structs: dll + 0x5033b90,
                player_info: dll + 0x5bc830c,
            },
            GameMode::HighGrade => Self {
                interaction: dll + 0x6b5fb7,
                player_structs: dll + 0xDC6B750,
                player_info: dll + 0xE7FFF3C,
            },
        }
    }

    fn player_info(&self) -> Option<PlayerInfo> {
        let ptr = unsafe { (self.player_info as *const *mut u8).read() };
        PlayerInfo::new(ptr)
    }

    pub fn own_player(&self) -> Option<Player> {
        let info = self.player_info()?;
        let idx = info.player_struct_idx() as usize;
        Some(Player::from_idx(self.player_structs as *mut u8, idx))
    }
}
