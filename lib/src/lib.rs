use eframe::egui;
use state::State;

mod state;

static mut STATE: Option<Box<State>> = None;
fn state(storage: &mut dyn eframe::Storage) -> &'static mut State {
    unsafe {
        match &mut STATE {
            Some(state) => state,
            None => {
                STATE = Some(Box::new(State::new(storage)));
                STATE.as_mut().unwrap()
            }
        }
    }
}

#[no_mangle]
pub fn update(ctx: &egui::Context, storage: &mut dyn eframe::Storage) {
    state(storage).update(ctx);
}

#[no_mangle]
pub fn reload_lib(storage: &mut dyn eframe::Storage) {
    state(storage).save(storage);
}
