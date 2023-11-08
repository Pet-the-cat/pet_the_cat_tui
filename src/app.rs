use std::{error, time::Instant};
use serde::{Serialize, Deserialize};

use crate::save;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub const MULTIPLIER_COST: u64 = 100;
pub const PETTING_MACHINE_COST: u64 = 300;

/// Application.
#[derive(Serialize, Deserialize, Debug)]
#[serde(default = "Default::default")]
pub struct App {
    /// Is the application running?
    #[serde(skip)]
    pub running: bool,
    /// Cat petted
    pub cat_petted: u64,
    // Cat petted multiplier
    pub multiplier: u64,
    // Cat petter machine
    pub cat_petting_machine: u64,
    // Time
    #[serde(skip, default = "Instant::now")]
    time: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            cat_petted: 0,
            multiplier: 1,
            cat_petting_machine: 0,
            time: Instant::now(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {

        // Pet cat by factory every second
        if self.cat_petting_machine > 0 {
            let time = self.time.elapsed().as_secs();
            if time > 0 {
                self.factory_pet_cat();
                self.time = Instant::now();
            }
        }

    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        save::save(self);
        
        self.running = false;
    }

    pub fn pet_cat(&mut self) {
        if let Some(res) = self.cat_petted.checked_add(self.multiplier) {
            self.cat_petted = res;
        }
    }
    
    pub fn factory_pet_cat(&mut self) {
        if let Some(res) = self.cat_petted.checked_add(self.cat_petting_machine) {
            self.cat_petted = res;
        }
    }

    pub fn buy_multiplier(&mut self) {
        if let Some(res) = self.cat_petted.checked_sub(MULTIPLIER_COST) {
            self.cat_petted = res;
            self.multiplier += 1;
        }
    }

    pub fn buy_factory(&mut self) {
        if let Some(res) = self.cat_petted.checked_sub(PETTING_MACHINE_COST) {
            self.cat_petted = res;
            self.cat_petting_machine += 1;
        }
    }
}
