#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

mod components;
mod libs;
mod state;
mod utils;

use dioxus::desktop::{ Config, LogicalSize, WindowBuilder };
use dioxus::prelude::*;
use utils::constants::APP_NAME;
use libs::ui;
use libs::window_manager::{ WindowAction, WINDOW_MANAGER };
use libs::input_manager::{ init_window_focus_state_with_value, get_window_focus_state };
use std::sync::mpsc;

#[cfg(target_os = "windows")]
const EMBEDDED_ICON: &[u8] = include_bytes!("../assets/icon.ico");

#[cfg(not(target_os = "windows"))]
const EMBEDDED_ICON: &[u8] = include_bytes!("../assets/icon.png");

fn load_icon() -> Option<dioxus::desktop::tao::window::Icon> {
    #[cfg(target_os = "windows")]
    let format = image::ImageFormat::Ico;

    #[cfg(not(target_os = "windows"))]
    let format = image::ImageFormat::Png;

    match image::load_from_memory_with_format(EMBEDDED_ICON, format) {
        Ok(img) => {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();

            #[cfg(target_os = "windows")]
            let target_size = 32u32;

            #[cfg(target_os = "linux")]
            let target_size = 64u32;

            #[cfg(target_os = "macos")]
            let target_size = 64u32;

            let final_rgba = if width != target_size || height != target_size {
                image::imageops::resize(&rgba, target_size, target_size, image::imageops::FilterType::Lanczos3)
            } else {
                rgba
            };

            match dioxus::desktop::tao::window::Icon::from_rgba(final_rgba.into_raw(), target_size, target_size) {
                Ok(icon) => Some(icon),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

fn main() {
    #[cfg(target_os = "windows")]
    if std::env::args_os().any(|arg| arg == libs::input_worker::WORKER_ARG) {
        libs::input_worker::run();
        return;
    }

    let cli = libs::cli_args::parse(std::env::args_os());

    #[cfg(target_os = "windows")]
    if cli.headless {
        libs::console_attach::attach_to_parent_console();
    }

    #[cfg(target_os = "windows")]
    let _instance_guard = match libs::single_instance::acquire() {
        Some(guard) => guard,
        None => {
            if cli.headless {
                always_eprint!(
                    "⚠️ {} is already running - close it first, or use the running copy",
                    APP_NAME
                );
                return;
            }
            libs::single_instance::signal_running_instance();
            always_eprint!("⚠️ {} is already running - raising its window instead", APP_NAME);
            return;
        }
    };

    #[cfg(target_os = "windows")]
    libs::single_instance::listen_for_wake_requests(|| {
        WINDOW_MANAGER.request_show();
    });

    utils::logger::init_debug_logging();
    env_logger::init();
    libs::trace::init();

    debug_print!("🚀 Initializing {}...", APP_NAME);

    let _manifest = state::manifest::AppManifest::load();

    if let Err(e) = state::paths::soundpacks::ensure_soundpack_directories() {
        debug_eprint!("⚠️ Failed to create soundpack directories: {}", e);
    }

    if cli.headless {
        libs::headless::run(&cli);
        return;
    }

    let startup_config = state::config_writer::current();
    let should_start_minimized =
        cli.minimized || (startup_config.auto_start && startup_config.start_minimized);

    state::app::init_app_state();
    state::app::init_update_state();

    utils::auto_updater::clear_stale_available_version();
    utils::auto_updater::restore_staged_update();

    state::ambiance::initialize_global_ambiance_player();

    let (keyboard_tx, keyboard_rx) = crossbeam_channel::unbounded::<String>();
    let (mouse_tx, mouse_rx) = crossbeam_channel::unbounded::<String>();
    let (hotkey_tx, hotkey_rx) = crossbeam_channel::unbounded::<String>();

    libs::audio::spawn_engine(keyboard_rx, mouse_rx, hotkey_rx);

    let initial_focus_state = !should_start_minimized;
    init_window_focus_state_with_value(initial_focus_state);

    libs::bootstrap::start_input_capture_with_focus(
        keyboard_tx,
        mouse_tx,
        hotkey_tx,
        get_window_focus_state()
    );

    let (window_tx, _window_rx) = mpsc::channel::<WindowAction>();
    WINDOW_MANAGER.set_action_sender(window_tx);

    let (window_width, default_height) = libs::window_bounds::DEFAULT_WINDOW_SIZE;
    let min_height = libs::window_bounds::MIN_WINDOW_SIZE.1;

    let window_icon = load_icon();

    #[cfg(target_os = "linux")]
    let window_builder = {
        WindowBuilder::default()
            .with_title(APP_NAME)
            .with_transparent(false)
            .with_always_on_top(false)
            .with_inner_size(LogicalSize::new(window_width, default_height))
            .with_min_inner_size(LogicalSize::new(window_width, min_height))
            .with_fullscreen(None)
            .with_decorations(false)
            .with_resizable(true)
            .with_visible(!should_start_minimized)
            .with_window_icon(window_icon)
    };

    #[cfg(not(target_os = "linux"))]
    let window_builder = WindowBuilder::default()
        .with_title(APP_NAME)
        .with_transparent(true)
        .with_always_on_top(false)
        .with_inner_size(LogicalSize::new(window_width, default_height))
        .with_min_inner_size(LogicalSize::new(window_width, min_height))
        .with_fullscreen(None)
        .with_decorations(false)
        .with_resizable(true)
        .with_visible(!should_start_minimized)
        .with_window_icon(window_icon);

    let config = Config::new()
        .with_window(window_builder)
        .with_menu(None);

    dioxus::LaunchBuilder::desktop().with_cfg(config).launch(app_with_stylesheets)
}

fn app_with_stylesheets() -> Element {
    rsx! {
        ui::app {}
    }
}
