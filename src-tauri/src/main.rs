// USB Fixer - Par Angel Virion
// Formate les clés USB en FAT32
// License: MIT

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Mutex;
use std::collections::HashSet;
use std::os::windows::process::CommandExt;
use std::io::Write;

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
fn format_drive(n: u32, state: tauri::State<State>, app: tauri::AppHandle) -> Result<String, String> {
    // Validation
    {
        let valid = state.0.lock().map_err(|_| "Erreur interne")?;
        if !valid.contains(&n) { return Err("Disque non autorisé".into()); }
    }
    if n > 99 { return Err("Numéro invalide".into()); }
    
    // Vérifie que c'est bien un USB
    let check = ps(&format!("(Get-Disk -Number {n}).BusType"));
    if check != "USB" { return Err("Ce n'est pas un disque USB".into()); }
    
    // Crée le script diskpart dans un fichier temp
    let diskpart_commands = format!(
        "select disk {n}\r\nclean\r\nconvert mbr\r\ncreate partition primary\r\nactive\r\nassign\r\n"
    );
    
    // Script PowerShell complet (identique à celui qui fonctionne)
    let ps_script = format!(r#"
$diskpartScript = @"
{diskpart_commands}"@
$diskpartScript | diskpart | Out-Null
Start-Sleep -Seconds 3
Update-Disk -Number {n} -ErrorAction SilentlyContinue
"#);

    // Exécute en admin
    ps(&format!(
        "Start-Process powershell -Verb runAs -WindowStyle Hidden -Wait -ArgumentList '-NoProfile -Command {}' ",
        ps_script.replace('\n', " ").replace('"', "'")
    ));
    
    // Attendre que Windows monte le volume
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Récupère la lettre
    let letter = ps(&format!(
        "(Get-Partition -DiskNumber {n} -EA SilentlyContinue | ?{{$_.DriveLetter}}).DriveLetter"
    ));
    
    if letter.is_empty() {
        return Err("Partition créée mais pas de lettre assignée".into());
    }
    
    // Ouvre HPUSBDisk en admin
    if let Some(p) = app.path_resolver().resolve_resource("HPUSBDisk.exe") {
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-Command", &format!("Start-Process '{}' -Verb runAs", p.display())])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    }
    
    Ok(format!("Prêt! Clé sur {}:", letter))
}

fn main() {
    tauri::Builder::default()
        .manage(State(Mutex::new(HashSet::new())))
        .invoke_handler(tauri::generate_handler![get_drives, format_drive])
        .run(tauri::generate_context!()).expect("Erreur");
}