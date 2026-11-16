use libva::{LibvaManager, VaDisplay, VaSurface, VaImageFormat};

fn main() {
    let mut mgr = LibvaManager::new("libva");
    
    let display = VaDisplay {
        id: "display0".to_string(),
        driver_name: "i965".to_string(),
        enabled: true,
    };
    
    if let Err(e) = mgr.add_display(display) {
        eprintln!("Failed to add display: {}", e);
        return;
    }
    
    let surface = VaSurface {
        id: "surface0".to_string(),
        width: 1920,
        height: 1080,
        format: VaImageFormat::NV12,
    };
    
    if let Err(e) = mgr.add_surface(surface) {
        eprintln!("Failed to add surface: {}", e);
        return;
    }
    
    println!("SigmaOS-libva operational: {} displays, {} surfaces", mgr.display_count(), mgr.surface_count());
}
