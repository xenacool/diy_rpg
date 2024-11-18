#[cfg(any(target_os = "android", target_os = "ios"))]
fn main() {}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn main() {
    let mut bevy_app = diy_rpg::create_bevy_app();
    bevy_app.run();
}
