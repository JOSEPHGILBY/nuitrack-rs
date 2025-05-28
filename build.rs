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

    let sdk_dest_path = manifest_dir.join(NUITRACK_SDK_VENDOR_SUBDIR);

    let desired_tag = get_desired_tag(&manifest_dir)?;
    println!("cargo:warning=Desired Nuitrack SDK tag: {}", desired_tag);

    if check_existing_sdk(&sdk_dest_path, &desired_tag) {
        println!(
            "cargo:warning=Nuitrack SDK version {} already vendored at {:?}. Skipping clone.",
            desired_tag, sdk_dest_path
        );
    } else {
        println!(
            "cargo:warning=Vendoring Nuitrack SDK tag {} into {:?}...",
            desired_tag, sdk_dest_path
        );
        setup_sdk_repo_with_git_command(&sdk_dest_path, &desired_tag)?;
        println!(
            "cargo:warning=Successfully cloned Nuitrack SDK tag {}.",
            desired_tag
        );
    }
    configure_native_build(&manifest_dir, &sdk_dest_path)?;  
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    Ok(())
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

    let project_bridge_h_dir = crate_root_dir.join("include").join("nuitrack_bridge");
    let project_bridge_rs_cc_dir = crate_root_dir.join("src").join("nuitrack_bridge");
    
    if !project_bridge_h_dir.is_dir() {
        bail!("Bridge modules include directory not found at: {:?}", project_bridge_h_dir);
    }
    if !project_bridge_rs_cc_dir.is_dir() {
        bail!("Bridge modules source directory not found at: {:?}", project_bridge_rs_cc_dir);
    }

    let mut rs_bridge_files = Vec::new();
    let mut cc_impl_files = Vec::new();

    println!("cargo:warning=Scanning for FFI headers in {:?} to find modules...", project_bridge_h_dir);
    for entry_result in fs::read_dir(&project_bridge_h_dir)? {
        let entry = entry_result?;
        let h_file_path = entry.path(); // This is the .h file path

        if !h_file_path.is_file() { continue; }

        let Some(extension) = h_file_path.extension() else { continue; };
        if extension != "h" { continue; } // Only process .h files

        let Some(stem) = h_file_path.file_stem().and_then(|s| s.to_str()) else {
            println!("cargo:warning=Skipping header file with non-UTF8 stem: {:?}", h_file_path.display());
            continue;
        };

        let rs_file = project_bridge_rs_cc_dir.join(format!("{}.rs", stem));
        let cc_file = project_bridge_rs_cc_dir.join(format!("{}.cc", stem));

        if rs_file.is_file() && cc_file.is_file() {
            println!("cargo:warning=Found FFI module corresponding to header: {}.h (stem: {})", stem, stem);
            rs_bridge_files.push(rs_file.clone());
            cc_impl_files.push(cc_file.clone());

            println!("cargo:rerun-if-changed={}", h_file_path.display());
            println!("cargo:rerun-if-changed={}", rs_file.display());
            println!("cargo:rerun-if-changed={}", cc_file.display());
        } else {
            if !rs_file.is_file() {
                println!("cargo:warning=Skipping header {} because corresponding Rust bridge file {}.rs was not found in {:?}",
                         h_file_path.display(), stem, project_bridge_rs_cc_dir);
            }
            if !cc_file.is_file() {
                println!("cargo:warning=Skipping header {} because corresponding C++ implementation {}.cc was not found in {:?}",
                         h_file_path.display(), stem, project_bridge_rs_cc_dir);
            }
        }
    }
    if rs_bridge_files.is_empty() {
        bail!("No Rust FFI bridge files (.rs) were successfully paired with .cc and .h files for compilation. Searched based on headers in {:?}.", project_bridge_h_dir);
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