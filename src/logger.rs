use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Phase {
  Initializing,
  Raytracing,
  Processing,
  Finishing,
}

pub struct Logger {
  pub phase: Phase,
  pub progress: f32,
}

impl Logger {
  pub fn new(phase: Phase, log: bool) -> Self {
    if log {
      println!("{}", json!({ "phase": phase, "progress": 0.0 }).to_string());
    }
    Self {
      phase,
      progress: 0.0,
    }
  }

  pub fn update(&mut self, progress: f32, log: bool) {
    self.progress = progress;
    if log {
      self.log();
    }
  }

  pub fn to_string(&self) -> String {
    json!({ "phase": self.phase, "progress": self.progress }).to_string()
  }

  pub fn log(&mut self) {
    println!("{}", self.to_string());
  }

  pub fn log_message(&mut self, message: Value) {
    println!("{}", json!({ "phase": self.phase, "progress": self.progress, "message": message.to_string() }).to_string());
  }
}

