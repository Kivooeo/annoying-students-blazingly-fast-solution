#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::System;

const FORBIDDEN_GAMES: [&str; 3] = ["RobloxPlayerBeta.exe", "Minecraft.exe", "javaw.exe"];

fn main() {
    let mut s = System::new_all();

    loop {
        s.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        s.processes().into_iter().for_each(|x| {
            if FORBIDDEN_GAMES.contains(&x.1.name().to_str().unwrap()) {
                x.1.kill();
            }
        });
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
