# USB Fixer

Formate les clés USB en FAT32 (compatible Konica Minolta et autres).

**Par Angel Virion** | [MIT License](LICENSE)

## Fonctionnalités

- ✅ Supprime la protection en écriture
- ✅ Formate en FAT32
- ✅ Compatible clés jusqu'à 32 GB

## Installation

```bash
npm install
npm run tauri dev      # Développement
npm run tauri build    # Production
```

## Sécurité

- Validation whitelist des disques USB
- Double vérification (Rust + PowerShell)
- Limite 32 GB (restriction FAT32 Windows)
- CSP activé, permissions minimales
