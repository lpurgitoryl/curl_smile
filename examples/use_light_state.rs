use curl_smile::{Intent, LightState};

fn main() {
    let mut state = LightState::new();

    state.update(Intent::Switch { on: true });
    state.update(Intent::Brightness { brightness: 128 });
    //state.update(Intent::Effect { effect: Effect::Jump7 });

    println!("{state:?}");
}