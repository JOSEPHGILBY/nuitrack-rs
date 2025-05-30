use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const NUITRACK_SDK_VENDOR_SUBDIR: &str = "vendor/nuitrack-sdk";
const NUITRACK_REPO_URL: &str = "https://github.com/3DiVi/nuitrack-sdk.git";

fn main() -> Result<()> {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR")
        .context("Failed to get CARGO_MANIFEST_DIR environment variable")?; // Apply context here, then unwrap with ?

    let manifest_dir = PathBuf::from(manifest_dir_string);

    let out_dir_string = env::var("OUT_DIR")
        .context("Failed to get OUT_DIR environment variable")?;
    let out_dir = PathBuf::from(out_dir_string);
    
    let sdk_dest_path = out_dir.join(NUITRACK_SDK_VENDOR_SUBDIR);

    let version_specifier = get_desired_tag(&manifest_dir)?;
    println!("cargo:warning=Desired Nuitrack SDK tag: {}", version_specifier);

    // ** NEW: Resolve the specific tag **
    let resolved_tag = resolve_actual_tag(NUITRACK_REPO_URL, &version_specifier)?;
    println!(
        "cargo:warning=Resolved Nuitrack SDK tag for operations: {}",
        resolved_tag
    );

    if check_existing_sdk(&sdk_dest_path, &resolved_tag) {
        println!(
            "cargo:warning=Nuitrack SDK version {} already vendored at {:?}. Skipping clone.",
            version_specifier, sdk_dest_path
        );
    } else {
        println!(
            "cargo:warning=Vendoring Nuitrack SDK tag {} into {:?}...",
            resolved_tag, sdk_dest_path
        );
        setup_sdk_repo_with_git_command(&sdk_dest_path, &resolved_tag)?;
        println!(
            "cargo:warning=Successfully cloned Nuitrack SDK tag {}.",
            resolved_tag
        );
    }
    configure_native_build(&manifest_dir, &sdk_dest_path)?;  
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    Ok(())
}

fn resolve_actual_tag(repo_url: &str, version_specifier: &str) -> Result<String> {
    // Check if version_specifier is already a full version (e.g., vX.Y.Z or X.Y.Z)
    let specifier_parts: Vec<&str> = version_specifier
        .strip_prefix('v')
        .unwrap_or(version_specifier)
        .split('.')
        .collect();

    if specifier_parts.len() == 3 && specifier_parts.iter().all(|p| p.parse::<u32>().is_ok()) {
        // Looks like a full version (e.g., v0.38.2 or 0.38.2 was specified).
        // We'll use this tag directly. Git clone will fail if it doesn't exist.
        // Ensure it starts with 'v' if the original specifier didn't but parts suggest a full version.
        let tag_to_use = if !version_specifier.starts_with('v') && version_specifier.matches('.').count() == 2 {
            format!("v{}", version_specifier)
        } else {
            version_specifier.to_string()
        };
        println!(
            "cargo:warning=Using specific tag from Cargo.toml: {}",
            tag_to_use
        );
        return Ok(tag_to_use);
    }

    // Assumed to be a partial version like "v0.38" or "0.38"
    // Normalize: ensure it starts with 'v' for consistency with Nuitrack tags.
    let mut base_prefix = version_specifier.to_string();
    if !version_specifier.starts_with('v') && specifier_parts.len() == 2 { // e.g. "0.38"
        base_prefix = format!("v{}", version_specifier); // "v0.38"
    }

    // Validate that the base_prefix is now in vX.Y format
    let base_prefix_parts: Vec<&str> = base_prefix.split('.').collect();
    if !(base_prefix.starts_with('v') && base_prefix_parts.len() == 2 && base_prefix_parts[0].len() > 1) {
         bail!(
            "Version specifier '{}' (normalized to '{}') is not in 'vX.Y' format for dynamic resolution or a full 'vX.Y.Z' tag. Nuitrack tags typically start with 'v' (e.g., v0.38).",
            version_specifier, base_prefix
        );
    }
    // Now base_prefix is like "v0.38"

    let glob_pattern = format!("refs/tags/{}*", base_prefix); // e.g., refs/tags/v0.38*

    println!(
        "cargo:warning=Attempting to resolve latest tag for prefix: {} using pattern: {}",
        base_prefix, glob_pattern
    );

    let output = Command::new("git")
        .args(["ls-remote", "--tags", "--refs", repo_url, &glob_pattern])
        .output()
        .context(format!("Failed to execute git ls-remote for {}", repo_url))?;

    if !output.status.success() {
        bail!(
            "git ls-remote command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let output_str =
        String::from_utf8(output.stdout).context("git ls-remote output was not valid UTF-8")?;

    let mut found_tags: Vec<String> = Vec::new();
    let expected_tag_start = format!("{}.", base_prefix); // e.g., "v0.38."

    for line in output_str.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 && parts[1].starts_with("refs/tags/") {
            let tag_name = parts[1].trim_start_matches("refs/tags/");
            // Ensure it strictly matches prefix + ".<number>" e.g. v0.38.0, v0.38.1 etc.
            if tag_name.starts_with(&expected_tag_start) {
                // Further check: ensure the part after "vX.Y." is numeric (patch version)
                let patch_part = tag_name.trim_start_matches(&expected_tag_start);
                if patch_part.parse::<u32>().is_ok() {
                    found_tags.push(tag_name.to_string());
                }
            }
        }
    }

    if found_tags.is_empty() {
        bail!(
            "No tags found matching the pattern '{}.Z' (e.g., '{}.0') in {}",
            base_prefix, base_prefix, repo_url
        );
    }

    found_tags.sort_by(|a, b| {
        let a_patch_str = a.trim_start_matches(&expected_tag_start);
        let b_patch_str = b.trim_start_matches(&expected_tag_start);
        let a_patch = a_patch_str.parse::<u32>().unwrap_or(0); // Default to 0 on parse error
        let b_patch = b_patch_str.parse::<u32>().unwrap_or(0);
        a_patch.cmp(&b_patch) // Sorts numerically in ascending order
    });

    let best_tag = found_tags.last().cloned().ok_or_else(|| {
        anyhow::anyhow!(
            "Failed to determine latest tag from candidates for prefix '{}'",
            base_prefix
        )
    })?;
    println!(
        "cargo:warning=Resolved tag for specifier '{}' to: {}",
        version_specifier, best_tag
    );
    Ok(best_tag)
}

fn get_desired_tag(crate_root_dir: &Path) -> Result<String> {
    let cargo_toml_path = crate_root_dir.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path)
        .context(format!("Failed to read {}", cargo_toml_path.display()))?;
    let cargo_toml_value: toml::Value = toml::from_str(&cargo_toml_content)
        .context(format!("Failed to parse {}", cargo_toml_path.display()))?;

    let tag = cargo_toml_value
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("nuitrack"))
        .and_then(|n| n.get("sdk_version_tag"))
        .and_then(|v| v.as_str())
        .context(
            "Key [package.metadata.nuitrack.sdk_version_tag] not found or not a string in Cargo.toml",
        )?
        .to_string();
    Ok(tag)
}

fn check_existing_sdk(sdk_path: &Path, desired_tag_from_cargo: &str) -> bool {
    if !sdk_path.is_dir() {
        return false;
    }
    let sdk_internal_version_file_path = sdk_path.join("VERSION");

    if !sdk_internal_version_file_path.is_file() {
        println!(
            "cargo:warning=Nuitrack SDK's internal VERSION file not found at {:?}. Assuming version mismatch or incomplete clone.",
            sdk_internal_version_file_path
        );
        return false; // Treat as mismatch to trigger re-clone if necessary
    }

    let content = match fs::read_to_string(&sdk_internal_version_file_path) {
        Ok(content) => content,
        Err(e) => {
            println!(
                "cargo:warning=Failed to read Nuitrack SDK's internal VERSION file at {:?}: {}. Assuming version mismatch.",
                sdk_internal_version_file_path, e
            );
            return false;
        }
    };

    let Some(sdk_reported_version_line) = content.lines().next() else {
        println!(
            "cargo:warning=Nuitrack SDK's internal VERSION file at {:?} is empty. Assuming version mismatch.",
            sdk_internal_version_file_path
        );
        return false;
    };

    let sdk_reported_version = sdk_reported_version_line.trim();
    let comparable_desired_version = desired_tag_from_cargo.strip_prefix('v').unwrap_or(desired_tag_from_cargo);

    if sdk_reported_version == comparable_desired_version {
        println!(
            "cargo:warning=Nuitrack SDK at {:?} reports version {} (from its VERSION file), which matches desired version {}.",
            sdk_path, sdk_reported_version, comparable_desired_version
        );
        return true;
    } else {
        println!(
            "cargo:warning=Version mismatch: SDK at {:?} reports version '{}' (from its VERSION file), but desired version is '{}' (from Cargo.toml tag '{}'). Will attempt to re-vendor.",
            sdk_path, sdk_reported_version, comparable_desired_version, desired_tag_from_cargo
        );
        return false;
    }
    
}

fn setup_sdk_repo_with_git_command(sdk_dest_path: &Path, desired_tag: &str) -> Result<()> {
    if sdk_dest_path.exists() {
        println!(
            "cargo:warning=Removing existing SDK directory: {:?}",
            sdk_dest_path
        );
        fs::remove_dir_all(&sdk_dest_path)
            .context(format!("Failed to remove old SDK dir: {:?}", sdk_dest_path))?;
    }
    if let Some(parent_dir) = sdk_dest_path.parent() {
        fs::create_dir_all(parent_dir).context(format!("Failed to create vendor directory: {:?}", parent_dir))?;
    }
    println!(
        "cargo:warning=Cloning Nuitrack SDK repository {} (tag {}) into {:?} using git.",
        NUITRACK_REPO_URL, desired_tag, sdk_dest_path
    );
    let clone_status = Command::new("git")
        .args([
            "clone",
            "--depth", "1",
            "--branch", desired_tag,
            NUITRACK_REPO_URL,
            sdk_dest_path.to_str().context("SDK destination path is not valid UTF-8")?,
        ])
        .status()
        .context("Failed to execute git clone command. Is git installed and in PATH?")?;
    if !clone_status.success() {
        bail!("git clone command failed with status: {}", clone_status);
    }
    let output = Command::new("git")
        .current_dir(sdk_dest_path)
        .args(["rev-parse", "HEAD"])
        .output()
        .context("Failed to execute git rev-parse HEAD")?;
    if !output.status.success() {
        bail!("git rev-parse HEAD failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    let commit_id = String::from_utf8(output.stdout)?.trim().to_string();
    if commit_id.is_empty() { bail!("git rev-parse HEAD returned empty commit ID"); }
    Command::new("git")
        .current_dir(sdk_dest_path)
        .args(["checkout", &commit_id])
        .status()
        .context(format!("Failed to execute git checkout {}", commit_id))?
        .success()
        .then_some(())
        .ok_or_else(|| anyhow::anyhow!("git checkout command failed"))?;
    println!("cargo:warning=Nuitrack SDK successfully cloned and checked out at commit {}.", &commit_id[0..std::cmp::min(7, commit_id.len())]);
    Ok(())
}

fn scan_for_modules_recursive(
    dir_to_scan: &Path,                // Current directory being scanned (e.g., .../include/nuitrack_bridge/types)
    base_include_dir: &Path,           // The top-level include dir (e.g., .../include/nuitrack_bridge)
    base_src_dir: &Path,               // The top-level src dir (e.g., .../src/nuitrack_bridge)
    rs_bridge_files: &mut Vec<PathBuf>,// Collector for .rs files
    cc_impl_files: &mut Vec<PathBuf>,  // Collector for .cc files
) -> Result<()> {
    for entry_result in fs::read_dir(dir_to_scan)
        .with_context(|| format!("Failed to read directory: {}", dir_to_scan.display()))? {
        let entry = entry_result
            .with_context(|| format!("Failed to read directory entry in: {}", dir_to_scan.display()))?;
        let current_path = entry.path();

        if current_path.is_dir() {
            scan_for_modules_recursive(
                &current_path,
                base_include_dir,
                base_src_dir,
                rs_bridge_files,
                cc_impl_files,
            )?;
            continue;
        } 
        if !current_path.is_file() { continue; }

            // Check if it's a header file
        if !(current_path.extension().and_then(std::ffi::OsStr::to_str) == Some("h")) { continue; }

        let h_file_path = current_path;

        // Get path relative to the base_include_dir (e.g., "core.h" or "types/skeleton.h")
        let relative_h_path = h_file_path.strip_prefix(base_include_dir)
            .with_context(|| format!("Failed to strip prefix from header path: {} relative to {}", h_file_path.display(), base_include_dir.display()))?;
        
        // Get the same relative structure but without the .h extension (e.g., "core" or "types/skeleton")
        let relative_stem_path = relative_h_path.with_extension(""); // Becomes "core" or "types/skeleton"
        
        let rs_file = base_src_dir.join(&relative_stem_path).with_extension("rs");
        let cc_file = base_src_dir.join(&relative_stem_path).with_extension("cc");

        if !(rs_file.is_file() && cc_file.is_file()) {
            if !rs_file.is_file() {
                println!(
                    "cargo:warning=Skipping header {} because corresponding Rust bridge file {} was not found",
                    h_file_path.display(), rs_file.display()
                );
            }
            if !cc_file.is_file() {
                println!(
                    "cargo:warning=Skipping header {} because corresponding C++ implementation {} was not found",
                    h_file_path.display(), cc_file.display()
                );
            }
            continue;
        } 

        println!(
            "cargo:warning=Found FFI module: .h: {}, .rs: {}, .cc: {}",
            h_file_path.display(), rs_file.display(), cc_file.display()
        );
        rs_bridge_files.push(rs_file.clone());
        cc_impl_files.push(cc_file.clone());

        println!("cargo:rerun-if-changed={}", h_file_path.display());
        println!("cargo:rerun-if-changed={}", rs_file.display());
        println!("cargo:rerun-if-changed={}", cc_file.display());
    }
    Ok(())
}

fn configure_native_build(
    crate_root_dir: &Path, // This is manifest_dir (e.g., .../nuitrack_rs)
    nuitrack_sdk_vendored_path: &Path, // This is .../nuitrack_rs/vendor/nuitrack-sdk
) -> Result<()> {
    let nuitrack_sdk_nuitrack_dir = nuitrack_sdk_vendored_path.join("Nuitrack");
    let nuitrack_sdk_include_dir = nuitrack_sdk_nuitrack_dir.join("include");

    if !nuitrack_sdk_include_dir.is_dir() {
        bail!("Vendored Nuitrack SDK missing include dir at: {:?}", nuitrack_sdk_include_dir);
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").context("CARGO_CFG_TARGET_OS not set")?;
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").context("CARGO_CFG_TARGET_ARCH not set")?;
    let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH")
        .context("CARGO_CFG_TARGET_POINTER_WIDTH not set")?;
    let platform_lib_dir_name = match target_os.as_str() {
        "windows" => match target_pointer_width.as_str() {
            "64" => "win64", "32" => "win32",
            _ => bail!("Unsupported pointer width for Windows: {}", target_pointer_width),
        },
        "linux" => match target_arch.as_str() {
            "x86_64" => "linux64", "aarch64" => "linux_arm64",
            arch if arch.starts_with("arm") => "linux_arm",
            _ => bail!("Unsupported architecture for Linux: {}", target_arch),
        },
        _ => bail!("Unsupported target OS for Nuitrack SDK: {}", target_os),
    };

    let nuitrack_sdk_lib_dir = nuitrack_sdk_nuitrack_dir.join("lib").join(platform_lib_dir_name);
    if !nuitrack_sdk_lib_dir.is_dir() {
        bail!("Vendored Nuitrack SDK missing platform library dir ('{}') at: {:?}", platform_lib_dir_name, nuitrack_sdk_lib_dir);
    }

    let project_bridge_h_base_dir = crate_root_dir.join("include").join("nuitrack_bridge");
    let project_bridge_src_base_dir = crate_root_dir.join("src").join("nuitrack_bridge");
    
    if !project_bridge_h_base_dir.is_dir() {
        bail!("Bridge modules include directory not found at: {:?}", project_bridge_h_base_dir);
    }
    if !project_bridge_src_base_dir.is_dir() {
        bail!("Bridge modules source directory not found at: {:?}", project_bridge_src_base_dir);
    }

    let mut rs_bridge_files = Vec::new();
    let mut cc_impl_files = Vec::new();

    println!("cargo:warning=Scanning for FFI headers in {:?} to find modules...", project_bridge_h_base_dir);
    scan_for_modules_recursive(
        &project_bridge_h_base_dir,    // dir_to_scan (starts at base)
        &project_bridge_h_base_dir,    // base_include_dir (for stripping prefix)
        &project_bridge_src_base_dir,  // base_src_dir (for joining relative paths)
        &mut rs_bridge_files,
        &mut cc_impl_files,
    )?;
    if rs_bridge_files.is_empty() {
        bail!("No Rust FFI bridge files (.rs) were successfully paired with .cc and .h files for compilation. Searched based on headers in {:?}.", project_bridge_h_base_dir);
    }

    // --- Configure cxx-build ---
    let mut build = cxx_build::bridges(&rs_bridge_files);
    build.files(&cc_impl_files)
        .include(&nuitrack_sdk_include_dir)      // For Nuitrack SDK's own headers (e.g., <nuitrack/Nuitrack.h>)
        .include(crate_root_dir.join("include")) // Base for your project's FFI headers (e.g., "nuitrack_bridge/core.h")
        .flag_if_supported("/EHsc")             // For MSVC C++ exceptions
        .std("c++17");

    if env::var("PROFILE").map_or(false, |p| p == "release") {
        build.define("NDEBUG", None);
    }

    build.compile("nuitrack_rs_ffi_glue"); // Name of the static lib cxx_build produces

    // --- Linking Nuitrack SDK library ---
    println!(
        "cargo:rustc-link-search=native={}",
        nuitrack_sdk_lib_dir.display()
    );
    println!("cargo:rustc-link-lib=dylib=nuitrack");
    Ok(())
}