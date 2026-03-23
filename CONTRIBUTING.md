# Guide de Contribution / Contributing Guide

Merci de votre intérêt pour USB Fixer! / Thank you for your interest in USB Fixer!

## 🇫🇷 Français

### Comment Contribuer

1. **Fork** le projet
2. Créez votre branche (`git checkout -b feature/AmeliorationIncroyable`)
3. Committez vos changements (`git commit -m 'Ajout: fonctionnalité incroyable'`)
4. Push vers la branche (`git push origin feature/AmeliorationIncroyable`)
5. Ouvrez une **Pull Request**

### Standards de Code

- **Rust:** Suivre les conventions Clippy (`clippy::all`, `clippy::pedantic`)
- **TypeScript:** Suivre les conventions ESLint
- **Commits:** Utilisez des messages clairs en français (préférable) ou anglais
  - Format: `Type: Description courte`
  - Types: `Ajout`, `Fix`, `Refactor`, `Docs`, `Test`, `Sécurité`

### Tests de Sécurité

Avant de soumettre une PR:
1. Exécutez `cargo clippy` sans avertissements
2. Testez sur une vraie clé USB
3. Vérifiez qu'aucune injection de commande n'est possible

### Normes Québécoises

Ce projet respecte:
- Loi 25 sur la protection des renseignements personnels
- Aucune collecte de données
- Code commenté en français (prioritaire) ou anglais

---

## 🇬🇧 English

### How to Contribute

1. **Fork** the project
2. Create your branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add: amazing feature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a **Pull Request**

### Code Standards

- **Rust:** Follow Clippy conventions (`clippy::all`, `clippy::pedantic`)
- **TypeScript:** Follow ESLint conventions
- **Commits:** Use clear messages in French (preferred) or English
  - Format: `Type: Short description`
  - Types: `Add`, `Fix`, `Refactor`, `Docs`, `Test`, `Security`

### Security Testing

Before submitting a PR:
1. Run `cargo clippy` with no warnings
2. Test on a real USB drive
3. Verify no command injection is possible

### Quebec Standards

This project complies with:
- Law 25 on personal information protection
- No data collection
- Code commented in French (priority) or English

---

## Code de Conduite / Code of Conduct

- Soyez respectueux / Be respectful
- Pas de discrimination / No discrimination
- Focus sur le code, pas sur les personnes / Focus on code, not people

## Questions?

Ouvrez une issue avec le tag `question` / Open an issue with the `question` tag
