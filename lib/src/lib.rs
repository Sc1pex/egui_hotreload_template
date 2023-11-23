use eframe::egui;

pub struct State {
    x: i32,
    y: i32,
}

impl State {
    pub fn new() -> Self {
        Self { x: 20, y: 10 }
    }
}

static mut STATE: Option<Box<State>> = None;

#[no_mangle]
pub fn state() -> &'static mut State {
    unsafe {
        match &mut STATE {
            Some(state) => state,
            None => {
                STATE = Some(Box::new(State::new()));
                STATE.as_mut().unwrap()
            }
        }
    }
}

#[no_mangle]
pub fn update(ctx: &egui::Context) {
    let state = state();

    egui::CentralPanel::default().show(&ctx, |ui| {
        ui.heading(format!("X is {}", state.x));

        if ui.button("Inc").clicked() {
            state.x += 1;
        }
        if ui.button("Decrement").clicked() {
            state.x -= 1;
        }

        ui.heading(format!("Y is {}", state.y));

        if ui.button("Inc").clicked() {
            state.y += 1;
        }
        if ui.button("Dec").clicked() {
            state.y -= 1;
        }
    });
}
