# Nombre Mystère (Rust)

Un petit jeu en ligne de commande écrit en Rust où le joueur doit deviner un nombre aléatoire compris entre **1 et 100**.

## Prérequis

- [Rust et Cargo](https://rustup.rs/) installés sur ta machine.

Vérifie ton installation :
```bash
rustc --version
cargo --version
```

## Installation et lancement

1. **Cloner le dépôt :**
   ```bash
   git clone [https://github.com/npautrat/mystery-number-rust-.git](https://github.com/npautrat/mystery-number-rust-.git)
   cd mystery-number-rust-
   ```

2. **Lancer le jeu :**
   ```bash
   cargo run
   ```
   *(Ou `cargo run --release` pour lancer une version optimisée).*

## Règles du jeu

- Le programme génère un nombre secret entre `1` et `100`.
- Saisis un nombre dans le terminal à chaque tour.
- Le jeu t'indique si ton estimation est trop petite ou trop grande jusqu'à ce que tu trouves la bonne réponse.
