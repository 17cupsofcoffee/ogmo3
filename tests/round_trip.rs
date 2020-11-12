use std::fs;

use ogmo3::{Level, Project};
use pretty_assertions::assert_eq;
use serde_json::Value;

#[test]
pub fn round_trip_project() {
    let input = fs::read_to_string("./examples/sample_project/test.ogmo").unwrap();
    let project = Project::from_json(&input).unwrap();

    // Serde always includes decimal places on floats, even if they're whole numbers,
    // so we have to hack around that to get output that matches what
    // Ogmo gives us.
    let output = project.to_json().unwrap().replace(".0", "");

    let input_json: Value = serde_json::from_str(&input).unwrap();
    let output_json: Value = serde_json::from_str(&output).unwrap();

    assert_eq!(input_json, output_json);
}

#[test]
pub fn round_trip_level() {
    let input = fs::read_to_string("./examples/sample_project/levels/uno.json").unwrap();
    let level = Level::from_json(&input).unwrap();

    // Serde always includes decimal places on floats, even if they're whole numbers,
    // so we have to hack around that to get output that matches what
    // Ogmo gives us.
    let output = level.to_json().unwrap().replace(".0", "");

    let input_json: Value = serde_json::from_str(&input).unwrap();
    let output_json: Value = serde_json::from_str(&output).unwrap();

    assert_eq!(input_json, output_json);
}
