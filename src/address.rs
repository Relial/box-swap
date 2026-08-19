use bunny_plugin::{GameMode, MhfoInfo};

#[derive(Clone, Copy, Debug)]
pub struct Addresses {
    pub interaction: usize,
}

impl Addresses {
    pub fn new(mhfo_info: MhfoInfo) -> Self {
        let dll = mhfo_info.address;
        match mhfo_info.game_mode {
            GameMode::LowGrade => Self {
                interaction: dll + 0x69b7f7,
            },
            GameMode::HighGrade => Self {
                interaction: dll + 0x6b5fb7,
            },
        }
    }
}
