fn main() {
    // For custom profiles, build-script env can be a bit confusing; make a best-effort
    // identification for our "iteration" profile (opt-level=1 + debuginfo).
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());
    let opt_level = std::env::var("OPT_LEVEL").unwrap_or_default();
    let debug = std::env::var("DEBUG").unwrap_or_default();

    let effective_profile = if profile == "debug" && opt_level == "1" && debug == "true" {
        "iteration".to_string()
    } else {
        profile
    };

    println!("cargo:rustc-env=OXIDE_PROFILE={effective_profile}");
}
