# USB Fixer

Formateur de cles USB en FAT32 avec suppression de la protection en ecriture.

Par **Angel Virion** | Licence MIT

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2-blue.svg)](https://tauri.app/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## Description

USB Fixer formate des cles USB en FAT32 sans limite de taille (>32GB) et supprime la protection en ecriture via diskpart. L'outil lance ensuite HPUSBDisk pour finaliser le formatage FAT32.

**Cas d'usage :**
- Cles USB protegees en ecriture
- Formatage FAT32 pour peripheriques (imprimantes, Konica Minolta, etc.)
- Preparation de cles USB pour usage professionnel

---

## Installation

### Option 1 : Executable (recommande)

Telechargez `USB-Fixer-v1.0.0.exe` depuis les [Releases](https://github.com/Legel42/UsbFixer/releases).

1. Double-cliquez sur l'executable
2. Acceptez l'invite UAC (droits admin requis)

### Option 2 : Compiler depuis le code source

**Prerequis :** Windows 10/11, Node.js 18+, Rust 1.70+

```bash
git clone https://github.com/Legel42/UsbFixer.git
cd UsbFixer
npm install
npm run tauri dev
```

Pour compiler en production :

```bash
npm run tauri build
```

L'executable sera dans `src-tauri/target/release/USB Fixer.exe`.

---

## Utilisation

1. Brancher la cle USB
2. Lancer USB Fixer
3. Selectionner la cle a formater
4. Cliquer sur **Formater la cle**
5. HPUSBDisk s'ouvre automatiquement pour finaliser le formatage FAT32

**Attention :** toutes les donnees sur la cle seront effacees.

---

## Fonctionnement

Le formatage se fait en deux etapes :

1. **diskpart** (via elevation admin) : `clean` > `convert mbr` > `create partition primary` > `active` > `assign`
2. **HPUSBDisk** : formatage FAT32 (lance automatiquement en admin)

La commande `clean` de diskpart supprime la table de partition, ce qui contourne la plupart des protections logicielles en ecriture.

---

## Securite

- Validation stricte des entrees (OWASP)
- Whitelist des disques autorises (USB uniquement)
- Double verification USB (prevention TOCTOU)
- Content Security Policy active
- Aucune collecte de donnees, 100% local

---

## Technologies

- **Backend :** Rust + Tauri 2
- **Frontend :** React + TypeScript
- **Build :** Vite
- **Formatage :** diskpart + HPUSBDisk

---

## Contribution

Les contributions sont les bienvenues. Ouvrez une issue ou un pull request.

- Tester sur materiel reel
- Respecter les conventions Clippy (Rust)
- Aucune collecte de donnees toleree

---

## Licence

[MIT](LICENSE) — libre d'utilisation, modification et redistribution, y compris pour un usage commercial.

---

## Avertissement

Ce logiciel est fourni **tel quel, sans aucune garantie**. L'auteur n'est pas responsable des pertes de donnees. Faites toujours une sauvegarde avant de formater une cle USB.

---

**© 2026 Angel Virion**
