use crate::app::App;

// Load the `App` save file in TOML format
pub fn load() -> App {
    let app_str_result = std::fs::read_to_string("save.toml");

    let app_str = match app_str_result {
        Ok(app_str) => app_str,
        Err(_) => {
            let app = App::new();
            save(&app);
            return app;
        }
    };

    let app: App = toml::from_str(&app_str).unwrap();

    app
}

pub fn save(app: &App) {
    let save = toml::to_string(&app).unwrap();
    std::fs::write("save.toml", save).unwrap();
}