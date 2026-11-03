use anyhow::Result;
use clap::Parser;
use libva::libva;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_hw_accel: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = libva::Config { 
        hardware_acceleration: !cli.no_hw_accel, 
        ..Default::default() 
    };
    let mut component = libva::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-libva ready");
    Ok(())
}
