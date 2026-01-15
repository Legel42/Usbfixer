// USB Fixer - Par Angel Virion
// Formate les clés USB en FAT32 - Conforme Loi 25 (Québec)
// License: MIT | © 2025 Angel Virion

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc, clippy::similar_names, clippy::cast_precision_loss, clippy::needless_pass_by_value)]

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Mutex;
use std::collections::HashSet;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const MAX_DISK_NUMBER: u32 = 99;
const GB: u64 = 1_073_741_824;
const MB: u64 = 1_048_576;

struct State(Mutex<HashSet<u32>>);

#[derive(Serialize, Deserialize)]
struct UsbDrive {
    disk_number: u32,
    name: String,
    size: String,
    letter: Option<String>
}

#[derive(Deserialize)]
struct PsDisk {
    #[serde(rename = "N")] n: u32,
    #[serde(rename = "Name")] name: String,
    #[serde(rename = "Size")] size: u64,
    #[serde(rename = "Letter")] letter: Option<String>,
}

fn ps(cmd: &str) -> String {
    Command::new("powershell")
        .args(["-NoProfile", "-Command", cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

#[tauri::command]
fn get_drives(state: tauri::State<'_, State>) -> Vec<UsbDrive> {
    let json = ps(r"Get-Disk|?{$_.BusType -eq 'USB'}|%{[PSCustomObject]@{
        N=$_.Number;Name=$_.FriendlyName;Size=$_.Size
        Letter=(Get-Partition -DiskNumber $_.Number -EA SilentlyContinue|?{$_.DriveLetter}).DriveLetter|Select -First 1
    }}|ConvertTo-Json -Compress");

    if json.is_empty() || json == "null" {
        if let Ok(mut s) = state.0.lock() { s.clear(); }
        return vec![];
    }

    let json = if json.starts_with('[') { json } else { format!("[{json}]") };
    let Ok(disks) = serde_json::from_str::<Vec<PsDisk>>(&json) else { return vec![]; };

    if let Ok(mut s) = state.0.lock() {
        s.clear();
        s.extend(disks.iter().map(|d| d.n));
    }

    disks.into_iter().map(|d| UsbDrive {
        disk_number: d.n,
        name: d.name,
        size: if d.size >= GB {
            format!("{:.1} GB", d.size as f64 / GB as f64)
        } else {
            format!("{:.0} MB", d.size as f64 / MB as f64)
        },
        letter: d.letter
    }).collect()
}

#[tauri::command]
fn format_drive(n: u32, state: tauri::State<'_, State>, app: tauri::AppHandle) -> Result<String, String> {
    // Validation de sécurité (conforme OWASP)
    if n > MAX_DISK_NUMBER { return Err("Numéro de disque invalide".into()); }

    {
        let valid = state.0.lock().map_err(|_| "Erreur interne")?;
        if !valid.contains(&n) { return Err("Disque non autorisé".into()); }
    }

    // Double vérification USB (prévention TOCTOU)
    let bus_type = ps(&format!("(Get-Disk -Number {n} -EA Stop).BusType 2>$null"));
    if bus_type != "USB" { return Err("Ce n'est pas un disque USB".into()); }

    // Crée le script diskpart de manière sécurisée
    let diskpart_path = std::env::temp_dir().join(format!("usbfixer_{n}.txt"));
    let script = format!("select disk {n}\r\nclean\r\nconvert mbr\r\ncreate partition primary\r\nactive\r\nassign\r\n");

    std::fs::write(&diskpart_path, &script)
        .map_err(|_| "Impossible de créer le script temporaire")?;

    // Exécution sécurisée de diskpart
    let script_path = diskpart_path.display().to_string();
    let _ = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            &format!("Start-Process -FilePath 'diskpart.exe' -ArgumentList '/s','{script_path}' -Verb RunAs -WindowStyle Hidden -Wait")])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|_| "Échec de l'exécution de diskpart")?;

    std::thread::sleep(std::time::Duration::from_secs(3));
    ps(&format!("Update-Disk -Number {n} -EA SilentlyContinue"));
    let _ = std::fs::remove_file(&diskpart_path);

    // Récupère la lettre de lecteur
    std::thread::sleep(std::time::Duration::from_secs(2));
    let letter = ps(&format!("(Get-Partition -DiskNumber {n} -EA SilentlyContinue|?{{$_.DriveLetter}}).DriveLetter"));

    if letter.is_empty() {
        return Err("Partition créée mais aucune lettre assignée".into());
    }

    // Lance HPUSBDisk de manière sécurisée
    if let Some(exe_path) = app.path_resolver().resolve_resource("HPUSBDisk.exe") {
        // Validation du chemin (prévention Path Traversal)
        if exe_path.file_name().and_then(|n| n.to_str()) == Some("HPUSBDisk.exe") {
            let path_str = exe_path.display().to_string();
            let _ = Command::new("powershell")
                .args(["-NoProfile", "-Command", &format!("Start-Process '{path_str}' -Verb runAs")])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn();
        }
    }

    Ok(format!("Prêt! Clé sur {letter}:"))
}

fn main() {
    tauri::Builder::default()
        .manage(State(Mutex::new(HashSet::new())))
        .invoke_handler(tauri::generate_handler![get_drives, format_drive])
        .run(tauri::generate_context!()).expect("Erreur");
}