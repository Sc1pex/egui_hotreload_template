use eframe::{CreationContext, NativeOptions};

#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    hot_functions_from_file!("lib/src/lib.rs");

    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}

    pub use eframe::egui;
    pub use lib::State;
}

static mut IS_RELOADING: bool = false;

struct MainApp {}
impl MainApp {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {}
    }
}
impl eframe::App for MainApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        unsafe {
            if IS_RELOADING {
                return;
            }
        }

        hot_lib::update(ctx);
        // ctx.request_repaint();
    }
}

fn main() {
    std::thread::spawn(|| loop {
        let token = hot_lib::subscribe().wait_for_about_to_reload();
        unsafe {
            IS_RELOADING = true;
        }
        drop(token);

        hot_lib::subscribe().wait_for_reload();
        unsafe {
            IS_RELOADING = false;
        }
    });

    let native_options = NativeOptions::default();
    eframe::run_native(
        "Hot reloadable",
        native_options,
        Box::new(|cc| Box::new(MainApp::new(cc))),
    )
    .expect("Failed to run app");
}
