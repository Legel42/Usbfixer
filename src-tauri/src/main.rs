// USB Fixer - Backend Tauri
// Par Angel Virion
// Licence: MIT

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::process::Command;
use std::sync::Mutex;
use tauri::State;

// =============================================================================
// CONSTANTES DE SÉCURITÉ
// =============================================================================

const MAX_USB_DRIVES: usize = 10;       // Limite anti-DoS
const MAX_DISK_NUMBER: u32 = 99;        // Numéro de disque max valide

// =============================================================================
// TYPES
// =============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsbDrive {
    disk_number: u32,
    friendly_name: String,
    size_formatted: String,
    drive_letter: Option<String>,
    is_readonly: bool,
}

#[derive(Deserialize)]
struct PsDisk {
    #[serde(rename = "DiskNumber")]
    disk_number: u32,
    #[serde(rename = "FriendlyName")]
    friendly_name: String,
    #[serde(rename = "Size")]
    size: u64,
    #[serde(rename = "IsReadOnly")]
    is_readonly: bool,
    #[serde(rename = "DriveLetter")]
    drive_letter: Option<String>,
}

// État partagé pour stocker les disques USB validés
struct AppState {
    valid_usb_disks: Mutex<HashSet<u32>>,
}

// =============================================================================
// FONCTIONS UTILITAIRES
// =============================================================================

fn format_size(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    match bytes {
        b if b >= GB => format!("{:.1} GB", b as f64 / GB as f64),
        b if b >= MB => format!("{:.1} MB", b as f64 / MB as f64),
        b => format!("{} B", b),
    }
}

fn run_powershell(script: &str) -> Result<String, String> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", script])
        .output()
        .map_err(|_| "Erreur système")?;

    if !output.status.success() {
        return Err("Échec de la commande".into());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn to_usb_drive(disk: PsDisk) -> UsbDrive {
    UsbDrive {
        disk_number: disk.disk_number,
        friendly_name: disk.friendly_name,
        size_formatted: format_size(disk.size),
        drive_letter: disk.drive_letter,
        is_readonly: disk.is_readonly,
    }
}

// =============================================================================
// COMMANDES TAURI
// =============================================================================

#[tauri::command]
async fn get_usb_drives(state: State<'_, AppState>) -> Result<Vec<UsbDrive>, String> {
    let script = r#"
        $disks = Get-Disk | Where-Object { $_.BusType -eq 'USB' } | Select-Object -First 10
        $result = @()
        foreach ($d in $disks) {
            $letter = (Get-Partition -DiskNumber $d.Number -EA SilentlyContinue | 
                       Where-Object { $_.DriveLetter }).DriveLetter | Select-Object -First 1
            $result += [PSCustomObject]@{
                DiskNumber = $d.Number
                FriendlyName = $d.FriendlyName
                Size = $d.Size
                IsReadOnly = $d.IsReadOnly
                DriveLetter = $letter
            }
        }
        $result | ConvertTo-Json -Compress
    "#;

    let output = run_powershell(script)?;

    if output.is_empty() || output == "null" {
        let mut valid = state.valid_usb_disks.lock().map_err(|_| "Erreur interne")?;
        valid.clear();
        return Ok(Vec::new());
    }

    let drives: Vec<UsbDrive> = if output.starts_with('[') {
        let disks: Vec<PsDisk> = serde_json::from_str(&output).map_err(|_| "Erreur de données")?;
        disks.into_iter().take(MAX_USB_DRIVES).map(to_usb_drive).collect()
    } else {
        let disk: PsDisk = serde_json::from_str(&output).map_err(|_| "Erreur de données")?;
        vec![to_usb_drive(disk)]
    };

    // Stocker les numéros de disque valides
    let mut valid = state.valid_usb_disks.lock().map_err(|_| "Erreur interne")?;
    valid.clear();
    for drive in &drives {
        valid.insert(drive.disk_number);
    }

    Ok(drives)
}

#[tauri::command]
async fn fix_usb_drive(
    disk_number: u32,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // VALIDATION 1: Numéro dans les limites
    if disk_number > MAX_DISK_NUMBER {
        return Err("Numéro de disque invalide".into());
    }

    // VALIDATION 2: Disque dans la liste des USB détectés
    {
        let valid = state.valid_usb_disks.lock().map_err(|_| "Erreur interne")?;
        if !valid.contains(&disk_number) {
            return Err("Disque non autorisé".into());
        }
    }

    // VALIDATION 3: Vérification côté PowerShell que c'est bien un USB
    let format_script = format!(
        r#"
        $d = Get-Disk -Number {n} -EA Stop
        if ($d.BusType -ne 'USB') {{ throw 'Non USB' }}
        Clear-Disk -Number {n} -RemoveData -RemoveOEM -Confirm:$false -EA Stop
        Initialize-Disk -Number {n} -PartitionStyle MBR -EA Stop
        New-Partition -DiskNumber {n} -UseMaximumSize -IsActive -AssignDriveLetter -EA Stop
        "#,
        n = disk_number
    );

    let elevated_cmd = format!(
        "Start-Process powershell -Verb runAs -Wait -ArgumentList '-NoProfile -ExecutionPolicy Bypass -Command \"{}\"'",
        format_script.replace('"', "`\"").replace('\n', " ")
    );

    run_powershell(&elevated_cmd)?;

    let hp_path = app_handle
        .path_resolver()
        .resolve_resource("HPUSBDisk.exe")
        .ok_or("Ressource introuvable")?;

    Command::new(hp_path).spawn().map_err(|_| "Échec du lancement")?;

    Ok(())
}

// =============================================================================
// POINT D'ENTRÉE
// =============================================================================

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            valid_usb_disks: Mutex::new(HashSet::new()),
        })
        .invoke_handler(tauri::generate_handler![get_usb_drives, fix_usb_drive])
        .run(tauri::generate_context!())
        .expect("Erreur de démarrage");
}
