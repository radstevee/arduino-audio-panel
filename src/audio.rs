pub fn create_sink(name: String) {
    let command = format!(
        "pactl load-module module-null-sink media.class=Audio/Source/Virtual sink_name={} channel_map=front-left,front-right", name
    );
    std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
}

pub fn destroy_sink(name: String) {
    let command = format!(
        "pactl unload-module \"$(pactl list short modules | grep \"{}\" | awk '{{print $1}}')\"",
        name
    );
    std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
}

pub fn set_volume(name: String, volume: u8) {
    println!("Setting volume of {} to {}%", name, volume);

    let command = format!("pactl set-source-volume {} {}%", name, volume);
    std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
}
