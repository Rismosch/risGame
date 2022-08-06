use ris_data::info::package_info::PackageInfo;
use ris_log::log::LogMessage;

use crate::{
    bootstrapper::{bootstrap, GlobalContainer},
    gameloop::{run_one_frame, GameloopState},
};

pub struct Engine {
    global_container: GlobalContainer,
}

impl Engine {
    pub fn new(package_info: PackageInfo) -> Result<Engine, String> {
        let global_container = bootstrap(package_info)?;

        let app_info = format!("{}", global_container.app_info);
        ris_log::log::forward_to_appenders(LogMessage::Unconstructed(app_info));

        Ok(Engine { global_container })
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            let gameloop_state = run_one_frame(&mut self.global_container);

            match gameloop_state {
                GameloopState::Running => continue,
                GameloopState::WantsToQuit => break,
                GameloopState::Error(error) => return Err(error),
            }
        }

        Ok(())
    }
}
