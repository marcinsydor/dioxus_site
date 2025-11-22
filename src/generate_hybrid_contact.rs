use std::fs;
use std::path::Path;

mod generate_static;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  Generating hybrid contact page with WASM...");

    let output_dir = Path::new("static_output");
    let wasm_assets_dir = Path::new("target/dx/dioxus_site/release/web/public/assets");

    if !wasm_assets_dir.exists() {
        return Err(
            "WASM build not found. Please run 'dx build --release --features web' first".into(),
        );
    }

    generate_static::generate_hybrid_contact_page(output_dir, wasm_assets_dir)?;

    println!("✅ Hybrid contact page generation complete!");
    Ok(())
}
