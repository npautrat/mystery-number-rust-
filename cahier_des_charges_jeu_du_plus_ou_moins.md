# 🎯 Projet Rust : Le Jeu du Plus ou Moins

Bienvenue dans ton premier vrai projet en Rust ! L'objectif est de créer un jeu en ligne de commande où le joueur doit deviner un nombre secret généré aléatoirement par l'ordinateur.

---

## 🛠️ Préparation du projet

1. Ouvre ton terminal et crée un nouveau projet Cargo :
   ```bash
   cargo new jeu_devinette
   cd jeu_devinette
   ```
2. Pour générer un nombre aléatoire, tu auras besoin de la crate `rand`. Ajoute-la à ton projet avec cette commande :
   ```bash
   cargo add rand
   ```
3. Lance ton programme à tout moment pour tester :
   ```bash
   cargo run
   ```

---

## 📋 Cahier des charges & Barème (/ 20 points)

Chaque critère validé rapporte **2 points**. Coche les cases `[x]` au fur et à mesure de ton avancement.

### 1. Génération du nombre secret (2 pts)
- [ ] Le programme génère un nombre aléatoire compris entre **1 et 100** inclus à chaque lancement (indice : utilise `rand::thread_rng().gen_range(1..=100)` ou `rand::random_range(1..=100)` selon la version de `rand`).

### 2. Saisie utilisateur dans le terminal (2 pts)
- [ ] Le programme demande au joueur d'entrer un nombre et lit son entrée au clavier grâce à `std::io::stdin()`.

### 3. Gestion propre des erreurs de saisie (2 pts)
- [ ] Si l'utilisateur tape du texte (ex: `"bonjour"`) ou appuie juste sur Entrée au lieu d'écrire un nombre, le programme **ne plante pas** (pas de crash/`panic!`). Il affiche un message d'erreur clair et redemande un nombre.

### 4. Boucle de jeu principale (2 pts)
- [ ] Le jeu tourne dans une boucle (`loop` ou `while`) tant que le joueur n'a pas trouvé le bon nombre (ou épuisé ses essais).

### 5. Indications « Plus grand » ou « Plus petit » (2 pts)
- [ ] À chaque proposition valide, le programme compare la saisie au nombre secret et indique clairement si le nombre secret est **plus grand** ou **plus petit**.

### 6. Compteur de tentatives (2 pts)
- [ ] Le programme compte le nombre d'essais effectués par le joueur et affiche le score final lors de la victoire (ex : *"Bravo ! Trouvé en 6 essais."*).

### 7. Limite d'essais / Condition de défaite (2 pts)
- [ ] Le joueur a un nombre maximum d'essais (par exemple **10 essais**). S'il atteint la limite sans trouver, la partie s'arrête et le programme révèle le nombre secret.

### 8. Vérification des bornes (2 pts)
- [ ] Si le joueur entre un nombre en dehors de l'intervalle (inférieur à `1` ou supérieur à `100`), le programme le prévient sans lui compter d'essai perdu (ou en lui signalant son erreur).

### 9. Option « Rejouer » (2 pts)
- [ ] À la fin d'une partie (victoire ou défaite), le programme demande au joueur s'il veut rejouer (`o`/`n`). Si oui, une nouvelle partie démarre avec un **nouveau** nombre secret.

### 10. Propreté du code et compilation sans avertissements (2 pts)
- [ ] Le code est bien indenté, lisible, commenté aux endroits clés, et la commande `cargo check` ou `cargo run` n'affiche **aucun warning** (pas de variables inutilisées, etc.).

---

## 💡 Aide-mémoire (Syntaxe utile)

* **Lire le clavier :**
  ```rust
  use std::io;

  let mut saisie = String::new();
  io::stdin().read_line(&mut saisie).expect("Erreur de lecture");
  ```
* **Convertir un `String` en nombre (`u32`) sans crasher :**
  ```rust
  let nombre: u32 = match saisie.trim().parse() {
      Ok(num) => num,
      Err(_) => {
          println!("Veuillez entrer un nombre valide !");
          continue;
      }
  };
  ```

---

## 🚀 Rendu du projet

Quand tu as terminé (ou si tu bloques sur un point), copie-colle le contenu de ton fichier `src/main.rs` (et ton `Cargo.toml`) dans ta prochaine réponse pour obtenir ta note et des conseils d'amélioration !