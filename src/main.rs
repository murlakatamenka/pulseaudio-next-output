use owo_colors::OwoColorize;
use pulsectl::controllers::{AppControl, DeviceControl, SinkController};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sink_controller =
        SinkController::create().expect("Failed to connect to PulseAudio server.");

    let devices = sink_controller
        .list_devices()
        .expect("Could not get list of output devices.");

    match devices.len() {
        0 => {
            eprintln!("No output devices found.");
            std::process::exit(1);
        }
        1 => {
            eprintln!("Single output device found, nothing to do.");
            std::process::exit(1);
        }
        _ => {}
    }

    // find vector index of current default sink
    let default_sink_id = sink_controller
        .get_default_device()
        .expect("Failed to get default output device.")
        .index;
    let cur_idx = devices
        .iter()
        .position(|d| d.index == default_sink_id)
        .unwrap();

    // next sink after currently default one
    let next_sink_idx = (cur_idx + 1) % devices.len();
    let new_default_output = &devices[next_sink_idx];
    let new_default_output_name: &str = new_default_output.name.as_ref().unwrap();

    // set new default sink to next_sink
    sink_controller
        .set_default_device(new_default_output_name)
        .expect("Failed to switch to another output device.");

    println!(
        "Switched to {}",
        match new_default_output_name.to_ascii_lowercase() {
            m if m.contains("hdmi") => "MONITOR",
            h if h.contains("headset") => "HEADSET",
            _ => new_default_output_name,
        }
        .green()
        .bold()
    );

    // move applications to new_default_sink
    for app in sink_controller.list_applications()? {
        sink_controller.move_app_by_index(app.index, new_default_output.index)?;
    }

    Ok(())
}
