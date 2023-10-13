use sdl2::keyboard::Scancode;

use ris_asset::asset_loader;
use ris_asset::asset_loader::AssetLoaderGuard;
use ris_asset::loader::scenes_loader;
use ris_asset::loader::scenes_loader::Scenes;
use ris_asset::AssetId;
use ris_data::gameloop::frame_data::FrameDataCalculator;
use ris_data::gameloop::input_data::InputData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::gameloop::output_data::OutputData;
use ris_data::info::app_info::AppInfo;
use ris_jobs::job_system;
use ris_jobs::job_system::JobSystemGuard;
use ris_log::console_appender::ConsoleAppender;
use ris_log::log;
use ris_log::log::Appenders;
use ris_log::log::LogGuard;
use ris_log::log_level::LogLevel;
use ris_log::log_message::LogMessage;
use ris_util::error::RisResult;
use ris_video::video::Video;

use crate::appenders::file_appender::FileAppender;
use crate::gameloop::input_frame::InputFrame;
use crate::gameloop::logic_frame::LogicFrame;
use crate::gameloop::output_frame::OutputFrame;

#[cfg(debug_assertions)]
fn scenes_id() -> AssetId {
    AssetId::Directory(String::from("root.ris_scenes"))
}

#[cfg(not(debug_assertions))]
fn scenes_id() -> AssetId {
    AssetId::Compiled(0)
}

pub struct GodObject {
    pub app_info: AppInfo,
    pub frame_data_calculator: FrameDataCalculator,
    pub input_frame: InputFrame,
    pub logic_frame: LogicFrame,
    pub output_frame: OutputFrame,
    pub input_data: InputData,
    pub logic_data: LogicData,
    pub output_data: OutputData,
    pub scenes: Scenes,

    // guards
    pub asset_loader_guard: AssetLoaderGuard,
    pub job_system_guard: JobSystemGuard,
    pub log_guard: LogGuard,
}

impl GodObject {
    pub fn new(app_info: AppInfo) -> RisResult<Self> {
        // logging
        let log_level = LogLevel::Debug;
        let appenders: Appenders = vec![ConsoleAppender::new(), FileAppender::new(&app_info)];
        let log_guard = unsafe { log::init(log_level, appenders) };

        let formatted_app_info = format!("{}", &app_info);
        ris_log::log::forward_to_appenders(LogMessage::Plain(formatted_app_info));

        // putting log_guard into an Option allows that it may only be taken, when no errors during
        // building occured. in turn, this allows logging the error below
        let mut log_guard = Some(log_guard);
        let result = Self::build_god_object(app_info, &mut log_guard);
        if let Err(error) = &result {
            ris_log::fatal!("failed to build god object:\n    {}", error);
        }

        result
    }

    fn build_god_object(
        app_info: AppInfo,
        log_guard: &mut Option<LogGuard>,
    ) -> RisResult<Self> {
        // settings

        // job system
        let cpu_count = app_info.cpu.cpu_count;
        let workers = app_info.args.workers;
        let job_system_guard = unsafe { job_system::init(1024, cpu_count, workers) };

        // assets
        import_assets()?;
        let asset_loader_guard = unsafe { asset_loader::init(&app_info)? };

        // sdl
        let sdl_context =
            sdl2::init().map_err(|e| ris_util::new_err!("failed to init sdl2: {}", e))?;
        let event_pump = sdl_context
            .event_pump()
            .map_err(|e| ris_util::new_err!("failed to get event pump: {}", e))?;
        let controller_subsystem = sdl_context
            .game_controller()
            .map_err(|e| ris_util::new_err!("failed to get controller subsystem: {}", e))?;

        // scenes
        let scenes_id = scenes_id();
        let scenes_bytes = ris_util::unroll!(
            asset_loader::load(scenes_id).wait(),
            "failed to load ris_scenes"
        )?;
        let scenes = scenes_loader::load(&scenes_bytes)?;

        // video
        let video = Video::new(&sdl_context, scenes.material.clone())?;

        // gameloop
        let input_frame = InputFrame::new(event_pump, controller_subsystem);
        let logic_frame = LogicFrame::default();
        let output_frame = OutputFrame::new(video);

        let frame_data_calculator = FrameDataCalculator::default();
        let mut input_data = InputData::default();
        let logic_data = LogicData::default();
        let output_data = OutputData::default();

        input_data.keyboard.keymask[0] = Scancode::Return;
        input_data.keyboard.keymask[15] = Scancode::W;
        input_data.keyboard.keymask[16] = Scancode::S;
        input_data.keyboard.keymask[17] = Scancode::A;
        input_data.keyboard.keymask[18] = Scancode::D;
        input_data.keyboard.keymask[19] = Scancode::Up;
        input_data.keyboard.keymask[20] = Scancode::Down;
        input_data.keyboard.keymask[21] = Scancode::Left;
        input_data.keyboard.keymask[22] = Scancode::Right;
        input_data.keyboard.keymask[28] = Scancode::Kp8;
        input_data.keyboard.keymask[29] = Scancode::Kp2;
        input_data.keyboard.keymask[30] = Scancode::Kp4;
        input_data.keyboard.keymask[31] = Scancode::Kp6;

        // log
        let log_guard = ris_util::unroll_option!(log_guard.take(), "passed log guard was none",)?;

        // god object
        let god_object = GodObject {
            app_info,
            frame_data_calculator,
            input_frame,
            logic_frame,
            output_frame,
            input_data,
            logic_data,
            output_data,
            scenes,

            // guards
            asset_loader_guard,
            job_system_guard,
            log_guard,
        };

        Ok(god_object)
    }
}

#[cfg(debug_assertions)]
fn import_assets() -> RisResult<()> {
    ris_log::debug!("importing assets...");

    use ris_asset::asset_importer::*;
    import_all(DEFAULT_SOURCE_DIRECTORY, DEFAULT_TARGET_DIRECTORY)?;

    ris_log::debug!("assets imported!");
    Ok(())
}

#[cfg(not(debug_assertions))]
fn import_assets() -> Result<(), RisError> {
    Ok(())
}
