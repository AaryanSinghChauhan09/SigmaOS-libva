//! SigmaOS-libva: Video Acceleration API (VAAPI) model.
//!
//! Models VA displays, surfaces, codecs, profiles, image formats,
//! and hardware-accelerated video processing.

use std::collections::BTreeMap;
use std::fmt;

/// VA profile types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaProfile {
    MPEG2Simple,
    MPEG2Main,
    H264Baseline,
    H264Main,
    H264High,
    VC1Simple,
    VC1Main,
    VC1Advanced,
    H263Baseline,
    JPEGBaseline,
    VP8Version0_3,
    VP9Profile0,
    HEVCMain,
    HEVCMain10,
    None,
}

/// VA entrypoint types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaEntrypoint {
    VLD,
    IZZ,
    FEI,
    EncSlice,
    EncPicture,
    EncQP,
    None,
}

/// Image format types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaImageFormat {
    NV12,
    YV12,
    YUY2,
    UYVY,
    ARGB,
    RGBA,
    RGB565,
    BGRA,
    BGRX,
    None,
}

/// A VA surface.
#[derive(Debug, Clone)]
pub struct VaSurface {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub format: VaImageFormat,
}

/// A VA context.
#[derive(Debug, Clone)]
pub struct VaContext {
    pub id: String,
    pub profile: VaProfile,
    pub entrypoint: VaEntrypoint,
    pub width: u32,
    pub height: u32,
}

/// A VA display (connection to driver).
#[derive(Debug, Clone)]
pub struct VaDisplay {
    pub id: String,
    pub driver_name: String,
    pub enabled: bool,
}

/// Errors for the libva domain.
#[derive(Debug, PartialEq)]
pub enum LibvaError {
    EmptyName,
    InvalidDimensions,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for LibvaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::InvalidDimensions => write!(f, "width and height must be non-zero"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for LibvaError {}

/// libva manager.
pub struct LibvaManager {
    pub name: String,
    displays: BTreeMap<String, VaDisplay>,
    surfaces: BTreeMap<String, VaSurface>,
    contexts: BTreeMap<String, VaContext>,
}

impl LibvaManager {
    /// Create a new manager.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            displays: BTreeMap::new(),
            surfaces: BTreeMap::new(),
            contexts: BTreeMap::new(),
        }
    }

    /// Add a display with validation.
    pub fn add_display(&mut self, display: VaDisplay) -> Result<(), LibvaError> {
        if display.driver_name.is_empty() {
            return Err(LibvaError::EmptyName);
        }
        if self.displays.contains_key(&display.id) {
            return Err(LibvaError::Duplicate(display.id));
        }
        self.displays.insert(display.id.clone(), display);
        Ok(())
    }

    /// Retrieve a display by id.
    pub fn get_display(&self, id: &str) -> Option<&VaDisplay> {
        self.displays.get(id)
    }

    /// Count of displays.
    pub fn display_count(&self) -> usize {
        self.displays.len()
    }

    /// Add a surface with validation.
    pub fn add_surface(&mut self, surface: VaSurface) -> Result<(), LibvaError> {
        if surface.width == 0 || surface.height == 0 {
            return Err(LibvaError::InvalidDimensions);
        }
        if self.surfaces.contains_key(&surface.id) {
            return Err(LibvaError::Duplicate(surface.id));
        }
        self.surfaces.insert(surface.id.clone(), surface);
        Ok(())
    }

    /// Retrieve a surface by id.
    pub fn get_surface(&self, id: &str) -> Option<&VaSurface> {
        self.surfaces.get(id)
    }

    /// Count of surfaces.
    pub fn surface_count(&self) -> usize {
        self.surfaces.len()
    }

    /// Add a context with validation.
    pub fn add_context(&mut self, context: VaContext) -> Result<(), LibvaError> {
        if context.width == 0 || context.height == 0 {
            return Err(LibvaError::InvalidDimensions);
        }
        if self.contexts.contains_key(&context.id) {
            return Err(LibvaError::Duplicate(context.id));
        }
        self.contexts.insert(context.id.clone(), context);
        Ok(())
    }

    /// Retrieve a context by id.
    pub fn get_context(&self, id: &str) -> Option<&VaContext> {
        self.contexts.get(id)
    }

    /// Count of contexts.
    pub fn context_count(&self) -> usize {
        self.contexts.len()
    }

    /// Enable or disable a display.
    pub fn set_display_enabled(&mut self, id: &str, enabled: bool) -> Result<(), LibvaError> {
        self.displays
            .get_mut(id)
            .map(|disp| disp.enabled = enabled)
            .ok_or_else(|| LibvaError::NotFound(id.to_string()))
    }

    /// Clear all VA resources.
    pub fn clear(&mut self) {
        self.displays.clear();
        self.surfaces.clear();
        self.contexts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_display(id: &str, driver: &str) -> VaDisplay {
        VaDisplay {
            id: id.to_string(),
            driver_name: driver.to_string(),
            enabled: true,
        }
    }

    fn make_surface(id: &str, width: u32, height: u32) -> VaSurface {
        VaSurface {
            id: id.to_string(),
            width,
            height,
            format: VaImageFormat::NV12,
        }
    }

    #[test]
    fn test_create_manager() {
        let mgr = LibvaManager::new("libva");
        assert_eq!(mgr.name, "libva");
        assert_eq!(mgr.display_count(), 0);
    }

    #[test]
    fn test_add_display() {
        let mut mgr = LibvaManager::new("libva");
        let disp = make_display("disp1", "i965");
        assert!(mgr.add_display(disp).is_ok());
        assert_eq!(mgr.display_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut mgr = LibvaManager::new("libva");
        let disp = make_display("disp1", "");
        assert_eq!(mgr.add_display(disp), Err(LibvaError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut mgr = LibvaManager::new("libva");
        let disp = make_display("disp1", "i965");
        mgr.add_display(disp.clone()).unwrap();
        assert_eq!(mgr.add_display(disp), Err(LibvaError::Duplicate("disp1".to_string())));
    }

    #[test]
    fn test_add_surface() {
        let mut mgr = LibvaManager::new("libva");
        let surf = make_surface("surf1", 1920, 1080);
        assert!(mgr.add_surface(surf).is_ok());
        assert_eq!(mgr.surface_count(), 1);
    }

    #[test]
    fn test_invalid_dimensions_rejected() {
        let mut mgr = LibvaManager::new("libva");
        let surf = make_surface("surf1", 0, 1080);
        assert_eq!(mgr.add_surface(surf), Err(LibvaError::InvalidDimensions));
    }

    #[test]
    fn test_set_display_enabled() {
        let mut mgr = LibvaManager::new("libva");
        let disp = make_display("disp1", "i965");
        mgr.add_display(disp).unwrap();
        assert!(mgr.set_display_enabled("disp1", false).is_ok());
        assert!(!mgr.get_display("disp1").unwrap().enabled);
        assert_eq!(mgr.set_display_enabled("none", true), Err(LibvaError::NotFound("none".to_string())));
    }

    #[test]
    fn test_clear() {
        let mut mgr = LibvaManager::new("libva");
        mgr.add_display(make_display("disp1", "i965")).unwrap();
        mgr.add_surface(make_surface("surf1", 1920, 1080)).unwrap();
        mgr.clear();
        assert_eq!(mgr.display_count(), 0);
        assert_eq!(mgr.surface_count(), 0);
    }
}
