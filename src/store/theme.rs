use std::{ rc::Rc };
use serde::{ de::DeserializeOwned, Serialize, Deserialize };

use yewdux::{ prelude::*, storage::{ self, Area, StorageError } };
use web_sys::{ Storage };

fn get_storage(area: Area) -> Result<Storage, StorageError> {
	let window = web_sys::window().ok_or(StorageError::WindowNotFound)?;
	let storage = match area {
		Area::Local => window.local_storage(),
		Area::Session => window.session_storage(),
	};

	storage.map_err(StorageError::WebSys)?.ok_or(StorageError::StorageAccess(area))
}

fn save<T: Serialize>(key: &str, state: &T, area: Area) -> Result<(), StorageError> {
	let storage = get_storage(area)?;

	let value = &serde_json::to_string(state).map_err(StorageError::Serde)?;
	storage.set(key, value).map_err(StorageError::WebSys)?;

	Ok(())
}

fn load<T: DeserializeOwned>(key: &str, area: Area) -> Result<Option<T>, StorageError> {
	let storage = get_storage(area)?;

	let value = storage.get(key).map_err(StorageError::WebSys)?;

	match value {
		Some(value) => {
			let state = serde_json::from_str(&value).map_err(StorageError::Serde)?;

			Ok(Some(state))
		}
		None => Ok(None),
	}
}

#[derive(Default)]
pub struct StorageListener;
impl Listener for StorageListener {
	type Store = Theme;

	fn on_change(&mut self, state: Rc<Self::Store>) {
		if let Err(err) = save("theme", state.as_ref(), storage::Area::Local) {
			println!("Error saving state to storage: {:?}", err);
		}
	}
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Theme {
	pub color: String,
}

pub enum ThemeConfig {
	Dark,
	Light,
	Toggle,
}

impl Reducer<Theme> for ThemeConfig {
	fn apply(&self, mut theme: Rc<Theme>) -> Rc<Theme> {
		let state = Rc::make_mut(&mut theme);
		match self {
			ThemeConfig::Dark => {
				state.color = "dark".to_string();
			}
			ThemeConfig::Light => {
				state.color = "light".to_string();
			}
			ThemeConfig::Toggle => {
				state.color = (if state.color == "dark" { "light" } else { "dark" }).to_string();
			}
		}
		theme
	}
}

impl Store for Theme {
	fn new() -> Self {
		init_listener(StorageListener);

		load("theme", storage::Area::Local).ok().flatten().unwrap_or_default()
	}

	fn should_notify(&self, other: &Self) -> bool {
		self != other
	}
}