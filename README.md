# USB Fixer

Formate les clés USB protégées en écriture pour HP USB Disk Storage Format Tool.

**Par Angel Virion** | MIT License

## Installation

```bash
npm install
npm run tauri dev
```

Placer `HPUSBDisk.exe` dans `src-tauri/` avant de compiler.

## Sécurité

- ✅ Whitelist des disques USB détectés
- ✅ Double vérification USB (Rust + PowerShell)
- ✅ Limite numéro de disque (0-99)
- ✅ CSP activé (protection XSS)
- ✅ Permissions Tauri minimales
- ✅ Aucune donnée personnelle collectée

## Build

```bash
npm run tauri build
```
