use crate::{check_regular_file, fl, utils, G_HELLO_WINDOW};

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use gtk::prelude::{BuilderExtManual, WidgetExt};

use serde::Deserialize;
use subprocess::{Exec, Redirection};
use tracing::{error, info};

#[derive(Deserialize)]
struct Versions {
    #[serde(rename = "desktopISOVersion")]
    desktop_iso_version: String,
    #[serde(rename = "handheldISOVersion")]
    handheld_iso_version: String,
}

fn outdated_version_check(message: String) -> bool {
    let edition_tag: String =
        fs::read_to_string("/etc/edition-tag").unwrap_or("desktop".into()).trim().into();
    let version_tag: String =
        fs::read_to_string("/etc/version-tag").unwrap_or("testing".into()).trim().into();

    let window_ref = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().window };

    if version_tag.contains("testing") {
        utils::show_simple_dialog(
            window_ref,
            gtk::MessageType::Warning,
            &fl!("testing-iso-warning"),
            message.clone(),
        );
        return true;
    }

    let response = reqwest::blocking::get("https://cachyos.org/versions.json");

    if response.is_err() {
        utils::show_simple_dialog(
            window_ref,
            gtk::MessageType::Warning,
            &fl!("offline-error"),
            message.clone(),
        );
        return false;
    }

    let versions = response.unwrap().json::<Versions>().unwrap();

    let latest_version = if edition_tag.contains("desktop") {
        versions.desktop_iso_version
    } else {
        versions.handheld_iso_version
    }
    .trim()
    .to_owned();

    if version_tag != latest_version {
        utils::show_simple_dialog(
            window_ref,
            gtk::MessageType::Warning,
            &fl!("outdated-version-warning"),
            message.clone(),
        );
    }
    true
}

fn edition_compat_check(message: String) -> bool {
    let edition_tag = fs::read_to_string("/etc/edition-tag").unwrap_or("desktop".to_string());

    if edition_tag == "handheld" {
        let profiles_path =
            format!("{}/handhelds/profiles.toml", chwd::consts::CHWD_PCI_CONFIG_DIR);

        let handheld_profiles =
            chwd::profile::parse_profiles(&profiles_path).expect("Failed to parse profiles");
        let handheld_profile_names: Vec<_> =
            handheld_profiles.iter().map(|profile| &profile.name).collect();

        let available_profiles = chwd::profile::get_available_profiles(false);

        if available_profiles.iter().any(|profile| handheld_profile_names.contains(&&profile.name))
        {
            let window_ref = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().window };
            utils::show_simple_dialog(
                window_ref,
                gtk::MessageType::Warning,
                &fl!("unsupported-hw-warning"),
                message.clone(),
            );
            return true;
        }
    }
    true
}

fn connectivity_check(message: String) -> bool {
    let status = match reqwest::blocking::get("https://cachyos.org") {
        Ok(resp) => resp.status().is_success() || resp.status().is_server_error(),
        _ => false,
    };

    if !status {
        let window_ref = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().window };
        utils::show_simple_dialog(
            window_ref,
            gtk::MessageType::Error,
            &fl!("offline-error"),
            message,
        );
        return false;
    }
    true
}

pub fn launch_installer(message: String) {
    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        let builder = unsafe { &G_HELLO_WINDOW.as_ref().unwrap().builder };

        let install_btn: gtk::Button = builder.object("install").unwrap();
        install_btn.set_sensitive(false);

        let checks = [connectivity_check, edition_compat_check, outdated_version_check];
        if !checks.iter().all(|x| x(message.clone())) {
            // if any check failed, return
            info!("Some ISO check failed!");
            install_btn.set_sensitive(true);
            return;
        }

        // Spawning child process
        info!("ISO checks passed! Starting Installer..");
        let mut child = Exec::cmd("/usr/local/bin/calamares-online.sh")
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .popen()
            .expect("Failed to spawn installer");

        let child_out = child.stdout.take().unwrap();
        let child_read = BufReader::new(child_out);

        // Read the output line by line until EOF
        for line_result in child_read.lines() {
            match line_result {
                Ok(line) => info!("{line}"),
                Err(e) => error!("Error reading output: {e}"),
            }
        }

        let status = child.wait().expect("Failed to waiting for child");
        info!("Installer finished with status: {:?}", status);

        install_btn.set_sensitive(true);
    });
}

pub fn is_iso(preferences: &serde_json::Value) -> bool {
    Path::new(&preferences["live_path"].as_str().unwrap()).exists()
        && check_regular_file(preferences["installer_path"].as_str().unwrap())
}
