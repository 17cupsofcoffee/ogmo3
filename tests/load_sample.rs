use ogmo3::{Level, Project};

#[test]
pub fn load_sample_project() {
    Project::from_file("./examples/sample_project/test.ogmo").unwrap();
}

#[test]
pub fn load_sample_level() {
    Level::from_file("./examples/sample_project/levels/uno.json").unwrap();
}
