#![warn(missing_docs)]
//! System to specify and spawn waves of enemies

use bevy::{prelude::*, time::Stopwatch};
use rand::{seq::SliceRandom, Rng};

// TODO make spawn_id generic
pub struct SpawnEvent {
    pub spawn_id: String,
}

pub struct WaveInfo {
    pub spawn_pool: Vec<String>,
    pub spawn_count: u32,
}

pub struct WaveResource {
    wave_number: u32,
    wave_ongoing: bool,
    wave_timer: Stopwatch,
    spawns_left: u32,
    spawn_timer: Stopwatch,

    cooldown_period: f32,
    spawn_speed: f32,
    waves: Vec<WaveInfo>,
    paused: bool, // manual pausing (for debug)
}

impl WaveResource {
    pub fn total_waves(&self) -> u32 {
        self.waves.len() as u32
    }
    pub fn current_wave(&self) -> &WaveInfo {
        // TODO: will die if we have zero waves defined
        if self.wave_number == 0 || self.total_waves() == 0 {
            panic!("attempting to access wave zero or no waves");
        }
        let wave_number = self.wave_number.min(self.total_waves()) as usize;
        self.waves.get(wave_number - 1).unwrap()
    }
    pub fn pause(&mut self) {
        self.paused = true;
    }
    pub fn unpause(&mut self) {
        self.paused = false;
    }
    pub fn wave_number(&self) -> u32 {
        self.wave_number
    }
}

impl Default for WaveResource {
    fn default() -> Self {
        WaveResource {
            wave_number: 0,
            spawns_left: 0,
            wave_ongoing: false,
            wave_timer: Stopwatch::new(),
            spawn_timer: Stopwatch::new(),
            cooldown_period: 20.,
            spawn_speed: 1.,
            waves: vec![],
            paused: false,
        }
    }
}

fn wave_system(time: Res<Time>, mut res: ResMut<WaveResource>) {
    if res.paused {
        return;
    }

    // start new wave
    if res.spawns_left == 0 && res.wave_timer.elapsed_secs() > res.cooldown_period {
        res.wave_timer.pause();
        res.wave_timer.reset();

        res.wave_number += 1;
        res.spawns_left = res.current_wave().spawn_count;
        res.wave_ongoing = true;
    }

    res.wave_timer.tick(time.delta());
}

fn wave_spawn_system(
    time: Res<Time>,
    mut writer: EventWriter<SpawnEvent>,
    mut res: ResMut<WaveResource>,
) {
    if res.wave_ongoing == false || res.paused {
        return;
    }

    // spawn
    if res.spawn_timer.elapsed_secs() > res.spawn_speed {
        res.spawn_timer.reset();
        spawn(res.current_wave(), &mut writer);
        res.spawns_left -= 1;
    }

    // end wave
    if res.spawns_left == 0 {
        res.wave_ongoing = false;
        res.wave_timer.unpause();
    }

    res.spawn_timer.tick(time.delta());
}

fn spawn(current_wave: &WaveInfo, writer: &mut EventWriter<SpawnEvent>) {
    let spawn_id = current_wave.spawn_pool.choose(&mut rand::thread_rng());
    if spawn_id.is_none() {
        return;
    }
    let spawn_id = spawn_id.unwrap().clone();

    writer.send(SpawnEvent { spawn_id });
}
