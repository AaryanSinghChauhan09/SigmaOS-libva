use ::libva::{libva, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = libva::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.run().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-libva operational");
    Ok(())
}
