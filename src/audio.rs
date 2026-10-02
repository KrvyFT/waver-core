//! Device settings exchanged by the shell and GUI, independent of audio APIs.

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AudioSelection {
    pub backend: String,
    /// None follows the backend's default device.
    pub device: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AudioBackend {
    pub id: String,
    pub label: String,
    pub unavailable_reason: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AudioDevice {
    pub id: String,
    pub backend: String,
    pub label: String,
}

#[derive(Clone, Debug, Default)]
pub struct AudioCatalog {
    pub backends: Vec<AudioBackend>,
    pub devices: Vec<AudioDevice>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AudioSettingsRequest {
    Refresh,
    Apply(AudioSelection),
}
