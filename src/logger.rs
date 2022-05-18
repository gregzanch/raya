use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Phase {
  Initializing,
  Raytracing,
  Processing,
  Finishing,
}