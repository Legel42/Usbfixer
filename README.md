# USB Fixer

Déverrouille les clés USB protégées en écriture pour HP USB Disk Storage Format Tool.

**Par Angel Virion** | [MIT License](LICENSE)

## Installation

```bash
npm install
npm run tauri dev      # Développement
npm run tauri build    # Production
```

Placer `HPUSBDisk.exe` dans `src-tauri/` avant de compiler.

## Sécurité

- ✅ Validation des numéros de disque (whitelist)
- ✅ Double vérification USB (Rust + PowerShell)  
- ✅ Limite anti-DoS (max 10 disques)
- ✅ Messages d'erreur génériques (pas d'info leak)
- ✅ CSP activé (protection XSS)
- ✅ Permissions Tauri minimales

## Signaler une faille

Ouvrir une issue sur GitHub ou contacter l'auteur.
