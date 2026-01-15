# 🔧 USB Fixer

**Formateur de clés USB en FAT32 - Supprime la protection en écriture**

Par **Angel Virion** | License CC BY-NC 4.0 (Usage non-commercial uniquement)

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-1.5-blue.svg)](https://tauri.app/)
[![Loi 25](https://img.shields.io/badge/Loi_25-Conforme-green.svg)](PRIVACY_FR.md)

---

## 📋 Description

USB Fixer permet de formater des clés USB en FAT32 sans limite de taille (>32GB) et **supprime automatiquement la protection en écriture**. Développé spécifiquement pour les environnements de travail nécessitant des clés USB fiables.

**Cas d'usage:**
- Clés USB protégées en écriture
- Formatage FAT32 pour périphériques (Konica Minolta, imprimantes, etc.)
- Préparation de clés USB pour usage professionnel
- Clés >32GB nécessitant FAT32

---

## ⚡ Installation Rapide

### Prérequis
- Windows 10/11
- Node.js 18+ et npm
- Rust 1.70+ (pour compilation)

### Étapes

```bash
# 1. Cloner le dépôt
git clone https://github.com/[votre-username]/UsbFixer.git
cd UsbFixer

# 2. Installer les dépendances
npm install

# 3. Mode développement
npm run tauri dev

# 4. Compiler pour production
npm run tauri build
```

**Important:** Le fichier `HPUSBDisk.exe` est déjà inclus dans `src-tauri/`.

---

## 🚀 Utilisation

1. **Brancher** votre clé USB
2. **Lancer** USB Fixer
3. **Sélectionner** la clé à formater
4. Cliquer sur **"Formater"**
5. L'outil HPUSBDisk s'ouvre automatiquement pour finaliser le formatage FAT32

⚠️ **ATTENTION:** Toutes les données sur la clé seront effacées!

---

## ✨ Fonctionnalités

- ✅ **Suppression de la protection en écriture** (diskpart clean)
- ✅ Formatage FAT32 sans limite de taille (>32GB)
- ✅ Interface simple et rapide
- ✅ Validation stricte (disques USB uniquement)
- ✅ 100% local - aucune connexion Internet

---

## 🔒 Sécurité

### Mesures Implémentées
- ✅ **Validation stricte** des entrées (OWASP)
- ✅ **Whitelist** des disques autorisés
- ✅ **Double vérification USB** (prévention TOCTOU)
- ✅ **Protection Path Traversal**
- ✅ **Content Security Policy** activé
- ✅ **Aucune collecte de données**

### Conformité
- ✅ **Loi 25 (Québec)** - Aucune donnée personnelle collectée
- ✅ Clippy warnings (Rust best practices)
- ✅ Audit de sécurité documenté

Voir [SECURITY.md](SECURITY.md) pour plus de détails.

---

## 📊 Technologies

- **Backend:** Rust + Tauri 1.5
- **Frontend:** React + TypeScript
- **Build:** Vite
- **Formatage:** diskpart (Windows) + HPUSBDisk

---

## 🤝 Contribution

Les contributions sont bienvenues! Voir [CONTRIBUTING.md](CONTRIBUTING.md).

**Points importants:**
- Respecter les conventions Clippy (Rust)
- Tester sur matériel réel
- Aucune collecte de données tolérée
- Code commenté en français préférable

---

## 📜 Licence

**CC BY-NC 4.0** (Creative Commons - Attribution - Non Commercial)

- ✅ Usage personnel et éducatif
- ✅ Modification et redistribution (avec attribution)
- ❌ **Usage commercial INTERDIT** sans autorisation

Voir [LICENSE](LICENSE) pour les détails complets.

---

## 📞 Support

- **Issues:** [GitHub Issues](https://github.com/[votre-username]/UsbFixer/issues)
- **Sécurité:** Voir [SECURITY.md](SECURITY.md)
- **Confidentialité:** Voir [PRIVACY_FR.md](PRIVACY_FR.md)

---

## ⚠️ Avertissement

Ce logiciel est fourni **"TEL QUEL" SANS AUCUNE GARANTIE**. L'auteur n'est pas responsable des pertes de données ou dommages. **Utilisez à vos risques et périls.**

Toujours faire une **sauvegarde** avant de formater une clé USB!

---

**© 2025 Angel Virion** | Fabriqué au Québec 🇨🇦
