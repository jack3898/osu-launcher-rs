mod config;

use config::manager::LauncherConfig;

fn main() {
    let config = LauncherConfig::new("./launcher_config.json");

    println!("{}", config.config.osu_path);
}
