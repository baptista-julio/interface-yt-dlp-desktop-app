fn main() {
    // Expoe o target triple ao codigo via env!("YTDLP_TARGET_TRIPLE"),
    // usado para localizar os sidecars em modo dev (nome com sufixo de triple).
    let target = std::env::var("TARGET").unwrap_or_default();
    println!("cargo:rustc-env=YTDLP_TARGET_TRIPLE={target}");

    tauri_build::build()
}
