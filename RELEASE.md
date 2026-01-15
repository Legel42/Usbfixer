# 🚀 USB Fixer - Version Release

## 📦 Fichier Exécutable

**Fichier:** `USB-Fixer-v1.0.0.exe`
**Taille:** ~3.8 MB
**Plateforme:** Windows 10/11 (64-bit)

---

## ✅ Installation Simple

**Aucune installation requise!** C'est un exécutable standalone.

### Utilisation:

1. **Téléchargez** le fichier `USB-Fixer-v1.0.0.exe`
2. **Double-cliquez** sur l'exécutable
3. Acceptez l'invite UAC (droits administrateur requis)
4. L'application se lance!

---

## 🔒 Sécurité

### Pourquoi les droits administrateur?

USB Fixer nécessite des droits administrateur car il:
- Utilise `diskpart` pour supprimer la protection en écriture
- Modifie les partitions des clés USB
- Exécute des commandes système Windows

### Antivirus

Certains antivirus peuvent marquer l'application comme suspecte car:
- Elle demande des droits admin
- Elle modifie des disques
- C'est un exécutable non signé

**C'est normal et sécuritaire!** Le code source est disponible et audité.

---

## 📝 Notes Techniques

### Ce que contient l'exécutable:

- ✅ Application Tauri (Rust backend)
- ✅ Interface React (intégrée)
- ✅ HPUSBDisk.exe (formatage FAT32)
- ✅ Toutes les dépendances nécessaires

### Prérequis système:

- Windows 10 version 1809+ ou Windows 11
- WebView2 (installé automatiquement si manquant)
- Droits administrateur

---

## 🛠️ Compilation depuis le Code Source

Si vous préférez compiler vous-même:

```bash
# 1. Cloner le dépôt
git clone https://github.com/[votre-username]/UsbFixer.git
cd UsbFixer

# 2. Installer les dépendances
npm install

# 3. Compiler
npm run tauri build

# L'exécutable sera dans: src-tauri/target/release/USB Fixer.exe
```

---

## 🐛 Problèmes Courants

### L'application ne se lance pas

1. Vérifiez que vous avez Windows 10/11
2. Installez WebView2: https://developer.microsoft.com/microsoft-edge/webview2/
3. Exécutez en tant qu'administrateur (clic droit → "Exécuter en tant qu'administrateur")

### L'antivirus bloque l'exécution

1. Ajoutez une exception dans votre antivirus
2. Ou compilez depuis le code source (voir ci-dessus)

### La protection en écriture n'est pas enlevée

1. Vérifiez que vous avez accepté l'invite UAC
2. Certaines clés ont une protection matérielle (switch physique)
3. Essayez de débrancher/rebrancher la clé

---

## 📞 Support

- **Issues:** [GitHub Issues](https://github.com/[votre-username]/UsbFixer/issues)
- **Sécurité:** Voir [SECURITY.md](SECURITY.md)
- **Documentation:** Voir [README.md](README.md)

---

## 📜 Licence

**CC BY-NC 4.0** - Usage non-commercial uniquement
© 2025 Angel Virion

---

**⚠️ AVERTISSEMENT:** Ce logiciel efface toutes les données de la clé USB. Faites toujours une sauvegarde avant utilisation!
