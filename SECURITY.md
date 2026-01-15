# Politique de Sécurité / Security Policy

## Versions Supportées / Supported Versions

| Version | Support           |
| ------- | ----------------- |
| 1.0.x   | :white_check_mark: |

## Signaler une Vulnérabilité / Reporting a Vulnerability

**Français:**
Si vous découvrez une vulnérabilité de sécurité, veuillez **NE PAS** créer d'issue publique. Contactez directement l'auteur via GitHub Issues avec le tag `security` (en privé si possible).

**Aucun délai de réponse garanti** - ce projet est maintenu bénévolement. Les corrections seront apportées selon la disponibilité de l'auteur.

**English:**
If you discover a security vulnerability, please **DO NOT** create a public issue. Contact the author directly via GitHub Issues with the `security` tag (privately if possible).

**No guaranteed response time** - this project is maintained on a volunteer basis. Fixes will be provided based on author availability.

## Mesures de Sécurité Implémentées / Implemented Security Measures

### Protection contre les Injections
- ✅ Validation stricte des entrées utilisateur
- ✅ Aucune exécution de commandes arbitraires
- ✅ Validation des numéros de disque (0-99)

### Protection TOCTOU (Time-of-Check-Time-of-Use)
- ✅ Double vérification du type de bus USB
- ✅ Whitelist des disques autorisés

### Protection Path Traversal
- ✅ Validation stricte des chemins de fichiers
- ✅ Vérification du nom du fichier exécutable

### Content Security Policy (CSP)
- ✅ CSP activé dans Tauri
- ✅ Permissions minimales (all: false)

### Conformité Loi 25 (Québec)
- ✅ Aucune collecte de données personnelles
- ✅ Traitement local uniquement
- ✅ Aucune transmission de données

## Utilisation

⚠️ **Ce logiciel est fourni SANS GARANTIE.**
- Usage personnel et éducatif uniquement
- **Usage commercial INTERDIT** sans autorisation
- Voir LICENSE pour les détails

## Audit de Sécurité

Dernière révision: 2025-01-15
