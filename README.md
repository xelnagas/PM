# 🦀 PM - Plateforme de Gestion de Projets & Planification Avancée

PM est une application desktop moderne, ultra-rapide et autonome de gestion de projets et de planification temporelle. Développée en **Rust** avec **Tauri v2** et **SQLite**, elle intègre un moteur de calcul de graphe sans cycle (DAG) et la méthode du chemin critique (CPM).

---

## 📚 Documentation & Spécifications du Projet

| Document | Description |
| :--- | :--- |
| 📋 [**Cahier des Charges (CDC.md)**](./CDC.md) | Besoins métiers initiaux, périmètre global et contraintes techniques. |
| 📖 [**Spécifications Fonctionnelles (specificationfonctionel.md)**](./specificationfonctionel.md) | Description exhaustive de tous les workflows CRUD, cycles de vie, statuts et rôles. |
| 🏛️ [**Architecture Technique (archi.md)**](./archi.md) | Architecture en couches, schéma relationnel SQLite, moteur de graphe et IPC Tauri. |
| 🧪 [**Norme de Développement & TDD (normedev.md)**](./normedev.md) | Standards de code en Rust, pyramide des tests, property testing et pipeline CI. |
| 🚀 [**Plan de Développement (plandeveloppement.md)**](./plandeveloppement.md) | Feuille de route par phases, WBS, gestion des risques et critères d'acceptation. |
| 📘 [**Manuel Utilisateur (manuel.md)**](./manuel.md) | Guide pratique d'utilisation, prise en main du Gantt, Kanban et FAQ. |

---

## 🛠️ Stack Technologique

- **Core & Backend :** Rust 🦀 (édition 2021+)
- **Interface Graphique (GUI) :** Tauri v2 + TypeScript / Canvas 2D & SVG
- **Base de Données Locale :** SQLite 3 (via `sqlx` avec migrations embarquées)
- **Algorithmique :** Tri topologique de Kahn, Détection de cycles DAG, Critical Path Method (CPM)
- **Tests & Assurance Qualité :** TDD (`cargo test`), `proptest`, `criterion`, `cargo-mutants`, `cargo-llvm-cov`
