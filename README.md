# USB Fixer

Formate les clés USB en FAT32 (toutes tailles, compatible Konica Minolta).

**Par Angel Virion** | MIT License

## Installation

```bash
npm install
npm run tauri dev
```

**Important:** Placer `fat32format.exe` dans `src-tauri/` avant de compiler.

Télécharger: http://www.ridgecrop.co.uk/index.htm?guiformat.htm
(Télécharger "fat32format.exe" depuis la page)

## Fonctionnalités

- ✅ Formate en FAT32 (toutes tailles, pas de limite 32GB)
- ✅ Supprime la protection en écriture
- ✅ Compatible Konica Minolta

## Sécurité

- ✅ Whitelist des disques USB
- ✅ Double vérification USB
- ✅ CSP activé
- ✅ Permissions minimales

## Build

```bash
npm run tauri build
```
