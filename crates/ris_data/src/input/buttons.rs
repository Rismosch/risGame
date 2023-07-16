#[derive(Default, Clone)]
pub struct Buttons {
    up: u32,
    down: u32,
    hold: u32,
}

impl Buttons {
    pub fn up(&self) -> u32 {
        self.up
    }

    pub fn down(&self) -> u32 {
        self.down
    }

    pub fn hold(&self) -> u32 {
        self.hold
    }

    pub fn is_up(&self, actions: u32) -> bool {
        self.up & actions != 0
    }

    pub fn is_down(&self, actions: u32) -> bool {
        self.down & actions != 0
    }

    pub fn is_hold(&self, actions: u32) -> bool {
        self.hold & actions != 0
    }

    pub fn set(&mut self, new_state: &u32, old_state: &u32) {
        self.up = !new_state & old_state;
        self.down = new_state & !old_state;
        self.hold = *new_state;
    }

    pub fn update(&mut self, new_state: &u32) {
        self.up = !new_state & self.hold;
        self.down = new_state & !self.hold;
        self.hold = *new_state;
    }
}
