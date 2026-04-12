use serde_json::Value;
use std::process::Command;

#[test]
fn test_scrut_smoke() {
    // 1. Find scrut manifest path
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .output()
        .expect("failed to execute cargo metadata");

    let metadata: Value =
        serde_json::from_slice(&output.stdout).expect("failed to parse cargo metadata");
    let scrut_pkg = metadata["packages"]
        .as_array()
        .expect("packages field not found")
        .iter()
        .find(|p| p["name"] == "scrut")
        .expect("scrut package not found in metadata");

    let manifest_path = scrut_pkg["manifest_path"]
        .as_str()
        .expect("manifest_path not a string");

    // 2. Build tzbuddy first to ensure it's in target/debug
    let build_status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("failed to build tzbuddy");
    assert!(build_status.success());

    // 3. Add target/debug to PATH for scrut to find tzbuddy
    let path_env = std::env::var("PATH").unwrap_or_default();
    let target_dir = std::env::current_dir()
        .unwrap()
        .join("target")
        .join("debug");
    let new_path = if path_env.is_empty() {
        target_dir.to_string_lossy().to_string()
    } else {
        format!("{}:{}", target_dir.to_string_lossy(), path_env)
    };

    // 4. Run scrut test via cargo
    let status = Command::new("cargo")
        .env("PATH", new_path)
        .args([
            "run",
            "--manifest-path",
            manifest_path,
            "--",
            "test",
            "tests/smoke.md",
        ])
        .status()
        .expect("failed to run scrut via cargo");

    assert!(status.success());
}
