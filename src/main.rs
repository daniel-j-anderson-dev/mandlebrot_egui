mod app;

use app::{
    APP_NAME,
    app_creator,
    native_options
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(
        APP_NAME,
        native_options(),
        app_creator(),
    )
    .map_err(|eframe_error| eframe_error.into())
}