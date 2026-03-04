use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

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
