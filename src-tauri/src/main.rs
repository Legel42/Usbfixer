#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
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

#[derive(Serialize)]
struct UsbDrive { disk_number: u32, name: String, size: String, letter: Option<String> }

#[derive(Deserialize)]
struct PsDisk {
    #[serde(rename = "N")] n: u32,
    #[serde(rename = "Name")] name: String,
    #[serde(rename = "Size")] size: u64,
    #[serde(rename = "Letter")] letter: Option<String>,
}

fn ps(cmd: &str) -> String {
    Command::new("powershell")
        .args(["-NoProfile", "-NoLogo", "-NonInteractive", "-Command", cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

#[tauri::command]
fn get_drives(state: tauri::State<'_, State>) -> Vec<UsbDrive> {
    let json = ps(
        r"$d=Get-Disk|?{$_.BusType -eq 'USB'};if(!$d){return};$p=Get-Partition -EA 0|?{$_.DriveLetter};$d|%{$n=$_.Number;[PSCustomObject]@{N=$n;Name=$_.FriendlyName;Size=$_.Size;Letter=($p|?{$_.DiskNumber -eq $n}|Select -First 1).DriveLetter}}|ConvertTo-Json -Compress"
    );

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
        letter: d.letter,
    }).collect()
}

#[tauri::command]
fn format_drive(n: u32, state: tauri::State<'_, State>, app: tauri::AppHandle) -> Result<String, String> {
    if n > MAX_DISK_NUMBER { return Err("Numéro de disque invalide".into()); }

    {
        let valid = state.0.lock().map_err(|_| "Erreur interne")?;
        if !valid.contains(&n) { return Err("Disque non autorisé".into()); }
    }

    if ps(&format!("(Get-Disk -Number {n} -EA Stop).BusType 2>$null")) != "USB" {
        return Err("Ce n'est pas un disque USB".into());
    }

    let diskpart_path = std::env::temp_dir().join(format!("usbfixer_{n}.txt"));
    std::fs::write(&diskpart_path, format!("select disk {n}\r\nclean\r\nconvert mbr\r\ncreate partition primary\r\nactive\r\nassign\r\n"))
        .map_err(|_| "Impossible de créer le script temporaire")?;

    let script_path = diskpart_path.display().to_string();
    let _ = Command::new("powershell")
        .args(["-NoProfile", "-NoLogo", "-NonInteractive", "-Command",
            &format!("Start-Process diskpart.exe -ArgumentList '/s','{script_path}' -Verb RunAs -WindowStyle Hidden -Wait")])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let _ = std::fs::remove_file(&diskpart_path);
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Update-Disk + récupérer la lettre en un seul appel PS
    let letter = ps(&format!(
        "Update-Disk -Number {n} -EA 0;(Get-Partition -DiskNumber {n} -EA 0|?{{$_.DriveLetter}}).DriveLetter"
    ));

    if letter.is_empty() {
        return Err("Partition créée mais aucune lettre assignée".into());
    }

    if let Ok(dir) = app.path().resource_dir() {
        let exe = dir.join("HPUSBDisk.exe");
        if exe.exists() {
            let _ = Command::new("powershell")
                .args(["-NoProfile", "-NoLogo", "-NonInteractive", "-Command",
                    &format!("Start-Process '{}' -Verb RunAs", exe.display())])
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
