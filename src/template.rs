#![allow(dead_code)]

use std::fs::File;
use std::{
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Template {
    pub position: Point,
    health: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Template {
    pub fn new() -> Self {
        Self {
            position: Point::origin(),
            health: 100,
        }
    }

    pub fn load(path: PathBuf) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let deserialized: Self = serde_yaml_ng::from_reader(reader).unwrap();
        Ok(deserialized)
    }

    pub fn save(&self, path: PathBuf) -> io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        serde_yaml_ng::to_writer(writer, &self).unwrap();
        Ok(())
    }

    pub fn load_or_new(path: PathBuf) -> Self {
        let loaded_result = Self::load(path);

        if let Ok(template) = loaded_result {
            template
        } else {
            Self::new()
        }
    }

    pub fn health(self) -> usize {
        self.health
    }

    pub fn heal(&mut self, amount: usize) {
        self.health = std::cmp::min(amount + self.health, 100);
    }

    pub fn damage(&mut self, amount: usize) {
        self.health = 0_usize.max(self.health.saturating_sub(amount));
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Template {{ Position: ({}, {}), Health: {} }}",
            self.position.x, self.position.y, self.health
        )
    }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn move_steps(&mut self, x_steps: isize, y_steps: isize) {
        self.x += x_steps;
        self.y += y_steps;
    }
}
