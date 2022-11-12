# Repro Tauri

Minimal reproduction for https://github.com/tauri-apps/tauri/issues/5611

1. `cd src-ui && npm i`
2. `cd .. && cargo-tauri build`
3. `./src-tauri/target/release/repro-tauri.exe`
4. Close the application
5. Observe a background process is still running