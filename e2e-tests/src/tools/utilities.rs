use rusty_opengl::shaders::utils::get_current_dir_name;

pub fn get_path_to_shaders() -> String {
    if get_current_dir_name() == "e2e-tests" {
        String::from("shaders/")
    } else {
        String::from("e2e-tests/shaders/")
    }
}

pub fn get_path_to_python_scripts() -> String {
    if get_current_dir_name() == "e2e-tests" {
        String::from("src/tools/python_scripts/")
    } else {
        String::from("e2e-tests/src/tools/python_scripts/")
    }
}
