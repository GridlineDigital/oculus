use keyring::Entry;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub id: String,
    pub label: String,
    pub power: String,
    pub brightness: f64,
    pub color: Color,
    pub connected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub hue: f64,
    pub saturation: f64,
    pub kelvin: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub uuid: String,
    pub name: String,
    pub account: Account,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifxStatePayload {
    pub power: Option<String>,
    pub color: Option<String>,
    pub brightness: Option<f64>,
    pub duration: Option<f64>,
}

pub struct LifxState {
    pub client: Client,
    pub token: Mutex<Option<String>>, // Cache the token in memory
}

impl LifxState {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: Mutex::new(None),
        }
    }

    pub fn update_token(
        &self,
        token: String,
        _app: &tauri::AppHandle,
    ) -> Result<(), Box<dyn Error>> {
        // Update memory cache
        {
            let mut t = self.token.lock().unwrap();
            *t = Some(token.clone());
        }

        // Update keyring
        let entry = Entry::new("tauri-app-oculus", "lifx_api_token")?;
        if token.is_empty() {
            entry.delete_credential()?;
        } else {
            entry.set_password(&token)?;
        }

        Ok(())
    }

    fn validate_selector(selector: &str) -> Result<(), Box<dyn Error>> {
        // Basic validation: allow 'all', 'id:...', 'group:...', 'location:...'
        if selector == "all" {
            return Ok(());
        }
        // Allow raw alphanumeric IDs (e.g. "d073d52ca593")
        if selector.chars().all(|c| c.is_alphanumeric()) {
            return Ok(());
        }
        if selector.starts_with("id:")
            || selector.starts_with("group:")
            || selector.starts_with("location:")
            || selector.starts_with("label:")
        {
            // Check for valid characters (alphanumeric, hyphens, colons, underscores)
            if selector
                .chars()
                .all(|c| c.is_alphanumeric() || c == ':' || c == '-' || c == '_')
            {
                return Ok(());
            }
        }
        Err(format!("Invalid selector format: {}", selector).into())
    }

    pub async fn list_lights(&self, app: &tauri::AppHandle) -> Result<Vec<Light>, Box<dyn Error>> {
        let token = self.get_token_from_keyring_or_cache(app)?;
        let resp = self
            .client
            .get("https://api.lifx.com/v1/lights/all")
            .bearer_auth(token)
            .send()
            .await?
            .json::<Vec<Light>>()
            .await?;
        Ok(resp)
    }

    pub async fn set_state(
        &self,
        selector: &str,
        state: LifxStatePayload,
        app: &tauri::AppHandle,
    ) -> Result<(), Box<dyn Error>> {
        Self::validate_selector(selector)?;
        let token = self.get_token_from_keyring_or_cache(app)?;
        self.client
            .put(&format!(
                "https://api.lifx.com/v1/lights/{}/state",
                selector
            ))
            .bearer_auth(token)
            .json(&state)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn toggle_power(
        &self,
        selector: &str,
        app: &tauri::AppHandle,
    ) -> Result<(), Box<dyn Error>> {
        Self::validate_selector(selector)?;
        let token = self.get_token_from_keyring_or_cache(app)?;
        self.client
            .post(&format!(
                "https://api.lifx.com/v1/lights/{}/toggle",
                selector
            ))
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn list_scenes(&self, app: &tauri::AppHandle) -> Result<Vec<Scene>, Box<dyn Error>> {
        let token = self.get_token_from_keyring_or_cache(app)?;
        let resp = self
            .client
            .get("https://api.lifx.com/v1/scenes")
            .bearer_auth(token)
            .send()
            .await?
            .json::<Vec<Scene>>()
            .await?;
        Ok(resp)
    }

    pub async fn activate_scene(
        &self,
        uuid: &str,
        app: &tauri::AppHandle,
    ) -> Result<(), Box<dyn Error>> {
        // UUID validation could be added here too
        let token = self.get_token_from_keyring_or_cache(app)?;
        let response = self
            .client
            .put(&format!(
                "https://api.lifx.com/v1/scenes/scene_id:{}/activate",
                uuid
            ))
            .bearer_auth(token)
            .json(&serde_json::json!({}))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(format!(
                "Failed to activate scene: Status {}, Body: {}",
                status, text
            )
            .into());
        }

        Ok(())
    }

    fn get_token_from_keyring_or_cache(
        &self,
        _app: &tauri::AppHandle,
    ) -> Result<String, Box<dyn Error>> {
        // 1. Try cache
        {
            let t = self.token.lock().unwrap();
            if let Some(token) = &*t {
                return Ok(token.clone());
            }
        }

        // 2. Try keyring
        let entry = Entry::new("tauri-app-oculus", "lifx_api_token")?;
        match entry.get_password() {
            Ok(token) => {
                // Update cache
                let mut t = self.token.lock().unwrap();
                *t = Some(token.clone());
                Ok(token)
            }
            Err(e) => Err(format!(
                "API token error: {}. Please go to Settings to configure it.",
                e
            )
            .into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_lights_deserialization() {
        let json_response = r#"[
            {
                "id": "d073d5000001",
                "uuid": "00000000-0000-0000-0000-000000000000",
                "label": "Kitchen",
                "connected": true,
                "power": "on",
                "color": {
                    "hue": 0.0,
                    "saturation": 0.0,
                    "kelvin": 3500
                },
                "brightness": 1.0,
                "effect": "off",
                "group": {
                    "id": "12345",
                    "name": "Kitchen"
                },
                "location": {
                    "id": "67890",
                    "name": "Home"
                },
                "last_seen": "2023-10-27T10:00:00Z",
                "seconds_since_seen": 0,
                "product": {
                    "name": "LIFX Color 1000",
                    "identifier": "lifx_color_1000",
                    "company": "LIFX",
                    "capabilities": {
                        "has_color": true,
                        "has_variable_color_temp": true,
                        "has_ir": false,
                        "has_chain": false,
                        "has_matrix": false,
                        "has_multizone": false,
                        "min_kelvin": 2500,
                        "max_kelvin": 9000
                    }
                }
            }
        ]"#;

        let lights: Vec<Light> = serde_json::from_str(json_response).unwrap();
        assert_eq!(lights.len(), 1);
        assert_eq!(lights[0].label, "Kitchen");
        assert_eq!(lights[0].power, "on");
        assert_eq!(lights[0].brightness, 1.0);
    }
}
