# Changelog

Toutes les modifications notables de ce projet seront documentées dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.0.0/),
et ce projet adhère au [Semantic Versioning](https://semver.org/lang/fr/).

## [1.0.0] - 2025-01-15

### Ajouté
- Interface graphique Tauri + React
- Suppression automatique de la protection en écriture (diskpart clean)
- Formatage FAT32 sans limite de taille via HPUSBDisk
- Validation stricte des disques USB uniquement
- Double vérification USB (prévention TOCTOU)
- Protection contre les injections de commandes
- Protection Path Traversal
- Content Security Policy (CSP)
- Conformité Loi 25 (Québec) - aucune collecte de données
- Documentation complète (README, SECURITY, PRIVACY, CONTRIBUTING)
- Licence CC BY-NC 4.0 (usage non-commercial)

### Sécurité
- Validation OWASP des entrées utilisateur
- Whitelist des disques autorisés
- Aucune exécution de commandes arbitraires
- Audit Clippy (Rust best practices)
- Permissions minimales Tauri (all: false)

### Performance
- Optimisation du code Rust (141 → 134 lignes)
- Utilisation de constantes pour les valeurs magiques
- Pattern matching moderne (let-else)
- Réduction de la duplication de code

---

## Types de changements
- `Ajouté` pour les nouvelles fonctionnalités
- `Modifié` pour les changements aux fonctionnalités existantes
- `Déprécié` pour les fonctionnalités bientôt supprimées
- `Supprimé` pour les fonctionnalités supprimées
- `Corrigé` pour les corrections de bugs
- `Sécurité` pour les correctifs de vulnérabilités
