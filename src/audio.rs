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
    let command = format!("pactl unload-module \"$(pactl list short modules | grep \"{}\" | awk '{{print $1}}')\"", name);
    std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
}

pub fn set_volume(name: String, volume: f32) {
    println!("Setting volume of {} to {}", name, volume);

    // Format the volume with one decimal place
    let volume_str = format!("{:.1}", volume);

    let command = format!("pactl set-source-volume {} {}", name, volume_str);
    std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
}
