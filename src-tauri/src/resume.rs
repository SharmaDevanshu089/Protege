use crate::handle_error;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;
use windows_registry::CURRENT_USER;

#[derive(Deserialize, Debug)]
struct ProtegeConfig {
    #[serde(rename = "project-name")]
    project_name: Option<String>,
    #[serde(rename = "project-type")]
    project_type: Option<String>,
}

#[tauri::command]
pub fn load_project_env(folder_path: String) -> Result<(), String> {
    let toml_path = Path::new(&folder_path).join("protege.toml");

    if !toml_path.exists() {
        return Err("This is not a Protege folder".to_string());
    }

    let toml_content = fs::read_to_string(&toml_path)
        .map_err(|e| format!("Failed to read protege.toml: {}", e))?;

    let config: ProtegeConfig = toml::from_str(&toml_content)
        .map_err(|e| format!("Failed to parse protege.toml: {}", e))?;

    match (config.project_name, config.project_type) {
        (Some(name), Some(ptype)) => {
            env::set_var("PROJECT_NAME", name);
            env::set_var("PROJECT_TYPE", ptype);
            Ok(())
        }
        _ => Err("there is a probem with protege config.'.".to_string()),
    }
}

#[tauri::command]
pub fn resume_last_project(app_handle: tauri::AppHandle) {
    // 1. Try to open the registry key
    let key_result = CURRENT_USER.open("Software\\Protege");

    match key_result {
        Ok(key) => {
            // 2. Try to read the last_project_path string
            match key.get_string("last_project_path") {
                Ok(path) => {
                    // 3. Try to load the environment
                    match load_project_env(path) {
                        Ok(_) => {
                            println!("hello");
                        }
                        Err(e) => {
                            handle_error::create_dialog(app_handle, &e);
                        }
                    }
                }
                Err(_) => {
                    handle_error::create_dialog(
                        app_handle,
                        "Could not find 'last_project_path' in registry.(ERR_REG_NOT_FOUND)",
                    );
                }
            }
        }
        Err(_) => {
            handle_error::create_dialog(
                app_handle,
                "Could not fetch last project. (ERR_REG_NOT_RETURN)",
            );
        }
    }
}
