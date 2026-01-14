// USB Fixer - Par Angel Virion
// Formate les clés USB et ouvre HPUSBDisk
// License: MIT

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Mutex;
use std::collections::HashSet;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

struct State(Mutex<HashSet<u32>>);

#[derive(Serialize, Deserialize)]
struct UsbDrive { disk_number: u32, name: String, size: String, letter: Option<String> }

#[derive(Deserialize)]
struct PsDisk {
    #[serde(rename = "N")] n: u32,
    #[serde(rename = "Name")] name: String,
    #[serde(rename = "Size")] size: u64,
    #[serde(rename = "Letter")] letter: Option<String>,
}

// PowerShell sans fenêtre
fn ps(cmd: &str) -> String {
    Command::new("powershell")
        .args(["-NoProfile", "-Command", cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

#[tauri::command]
fn get_drives(state: tauri::State<State>) -> Vec<UsbDrive> {
    let json = ps(r#"Get-Disk|?{$_.BusType -eq 'USB'}|%{[PSCustomObject]@{
        N=$_.Number;Name=$_.FriendlyName;Size=$_.Size
        Letter=(Get-Partition -DiskNumber $_.Number -EA SilentlyContinue|?{$_.DriveLetter}).DriveLetter|Select -First 1
    }}|ConvertTo-Json -Compress"#);
    
    if json.is_empty() || json == "null" {
        if let Ok(mut s) = state.0.lock() { s.clear(); }
        return vec![];
    }
    
    let json = if json.starts_with('[') { json } else { format!("[{}]", json) };
    let disks: Vec<PsDisk> = serde_json::from_str(&json).unwrap_or_default();
    
    if let Ok(mut s) = state.0.lock() {
        s.clear();
        for d in &disks { s.insert(d.n); }
    }
    
    disks.into_iter().map(|d| UsbDrive {
        disk_number: d.n, name: d.name,
        size: if d.size >= 1073741824 { format!("{:.1} GB", d.size as f64 / 1073741824.0) } 
              else { format!("{:.0} MB", d.size as f64 / 1048576.0) },
        letter: d.letter
    }).collect()
}

#[tauri::command]
fn format_drive(n: u32, state: tauri::State<State>, app: tauri::AppHandle) -> Result<(), String> {
    {
        let valid = state.0.lock().map_err(|_| "Erreur interne")?;
        if !valid.contains(&n) { return Err("Disque non autorisé".into()); }
    }
    if n > 99 { return Err("Numéro invalide".into()); }
    
    // Formatage identique au script original (pipe)
    let script = format!(
        "Get-Disk -Number {n} | Clear-Disk -RemoveData -RemoveOEM -Confirm:$false -PassThru | Initialize-Disk -PartitionStyle MBR -PassThru; \
         New-Partition -DiskNumber {n} -UseMaximumSize -IsActive -AssignDriveLetter; \
         Start-Sleep -Seconds 2"
    );
    ps(&format!("Start-Process powershell -Verb runAs -WindowStyle Hidden -Wait -ArgumentList '-NoProfile -WindowStyle Hidden -Command {}'", script));
    
    // Ouvre HPUSBDisk en admin
    if let Some(p) = app.path_resolver().resolve_resource("HPUSBDisk.exe") {
        ps(&format!("Start-Process '{}' -Verb runAs", p.display()));
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(State(Mutex::new(HashSet::new())))
        .invoke_handler(tauri::generate_handler![get_drives, format_drive])
        .run(tauri::generate_context!()).expect("Erreur");
}
