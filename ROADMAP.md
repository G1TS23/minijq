# minijq — apprendre Rust en construisant un clone de `jq`

> Projet d'apprentissage. Objectif : maîtriser les fondamentaux de Rust en construisant un
> outil en ligne de commande utile, sur un domaine déjà connu (JSON).
>
> Profil de départ : Java + TypeScript, débutant complet en Rust.

**Environnement vérifié le 03/08/2026 :** `rustc` 1.94.1 · `cargo` 1.94.1 · `jq` 1.7.1 (le vrai, pour comparer les résultats)

---

## Rappel : c'est quoi `jq` ?

Un outil en ligne de commande pour fouiller et transformer du JSON — l'équivalent de `grep`/`sed`,
mais qui comprend la structure au lieu de voir du texte brut.

Avec un fichier `data.json` :

```json
{
  "users": [
    { "name": "Alice", "age": 30, "admin": true },
    { "name": "Bob",   "age": 25, "admin": false }
  ]
}
```

```bash
jq '.users[0].name' data.json            # → "Alice"
jq '.users[].name' data.json             # → "Alice"  "Bob"
jq '.users | length' data.json           # → 2
jq '.users[] | select(.admin)' data.json # → { "name": "Alice", ... }
```

La chaîne entre guillemets est écrite dans un petit langage propre à `jq`. Construire un clone
demande donc deux choses :

1. **Un lecteur de JSON** — transformer le texte en structure de données manipulable.
2. **Un lecteur de requêtes** — comprendre `.users[0].name`, puis l'appliquer.

On peut s'arrêter après la partie 1 et avoir déjà un projet complet.

---

## Les 3 règles du jeu

1. **Interdiction d'utiliser `serde_json`.** C'est la bibliothèque qui fait tout le travail à ma
   place, donc elle tue l'intérêt du projet. Elle servira dans les vrais projets, pas ici.
2. **Le code moche est autorisé.** `.clone()` partout, `.unwrap()` partout. Le nettoyage, c'est
   l'étape 7. Si je bloque 20 min sur une erreur du compilateur : je clone et j'avance.
3. **Lancer `cargo clippy` régulièrement.** Il relit le code et suggère des améliorations. Sur un
   projet d'apprentissage, c'est un prof gratuit.

---

## Étape 0 — Créer le projet

*≈ 15 min*

**Objectif :** un binaire qui affiche « hello ».

```bash
cd ~/Documents/Perso/cli
cargo new minijq
cd minijq
cargo run
```

**Ce que j'apprends :** l'anatomie d'un projet Rust.

| Fichier | Équivalent JS |
|---|---|
| `Cargo.toml` | `package.json` |
| `Cargo.lock` | `package-lock.json` |
| `src/main.rs` | point d'entrée |
| `target/` | le build (déjà ignoré par git) |

**Le piège à connaître tout de suite :** pour passer des arguments à *mon* programme et pas à cargo,
il faut `--` :

```bash
cargo run -- mon-fichier.json    # ✅
cargo run mon-fichier.json       # ❌ cargo croit que c'est pour lui
```

- [x] `cargo run` affiche quelque chose
- [x] j'ai créé un `data.json` de test à la racine (celui d'Alice et Bob ci-dessus)

---

## Étape 1 — Lire l'entrée

*≈ 1 h*

**Objectif :** `cargo run -- data.json` affiche le contenu du fichier tel quel.

**Ce que j'apprends :** ma première rencontre avec `Result`, c'est-à-dire la gestion d'erreurs sans
exceptions. Une fonction qui peut échouer le déclare dans son type de retour, et l'appelant est
*obligé* de traiter le cas.

**À chercher :**

- `std::env::args()` — récupérer les arguments
- `std::fs::read_to_string` — lire un fichier
- l'opérateur `?` — le raccourci « si erreur, remonte-la »
- `fn main() -> Result<(), Box<dyn std::error::Error>>` — pour pouvoir utiliser `?` dans `main`

**Le déclic à avoir.** Ces deux écritures sont identiques — comprendre pourquoi le `?` existe :

```
let contenu = match std::fs::read_to_string(chemin) {
    Ok(c) => c,
    Err(e) => return Err(e.into()),
};

let contenu = std::fs::read_to_string(chemin)?;
```

- [x] ça marche avec un fichier existant
- [x] ça affiche une erreur propre (sans crash brutal) avec un fichier inexistant

**Bonus :** lire depuis l'entrée standard quand aucun fichier n'est donné, pour pouvoir faire
`curl ... | minijq`. Chercher `std::io::stdin().read_to_string()`.

---

## Étape 2 — Décrire ce qu'est une valeur JSON

*≈ 30 min, mais c'est LE moment clé*

**Objectif :** définir le type qui représente n'importe quel JSON en mémoire. Zéro code exécutable,
que de la déclaration.

**Ce que j'apprends :** l'`enum` de Rust, l'outil le plus important du langage. C'est l'union
TypeScript (`type X = A | B | C`), mais vérifiée par le compilateur : impossible d'oublier un cas.

Créer `src/value.rs` et y écrire un `enum Value` avec 6 variants : `Null`, `Bool`, `Number`,
`String`, `Array`, `Object`. À moi de trouver ce que chacun doit contenir (indice : un tableau
contient des `Value`, un objet associe des `String` à des `Value`).

**À chercher :**

- `enum` avec données associées
- `Vec<T>` (= `Array<T>`) et `HashMap<K, V>` (= `Map`)
- `#[derive(Debug, Clone, PartialEq)]` à mettre au-dessus de l'enum. `Debug` permet d'afficher la
  structure avec `println!("{:#?}", ma_valeur)` — indispensable pour déboguer la suite.
- `mod value;` dans `main.rs` — le système de modules (un `import`, mais déclaratif)

**Question à me poser :** pourquoi `Array(Vec<Value>)` compile, alors qu'un variant qui contiendrait
un `Value` directement ne compilerait pas ? La réponse explique comment Rust range les choses en
mémoire — exactement ce que Java cache.

- [x] je peux construire à la main, dans `main.rs`, la valeur correspondant à `{"name": "Alice"}`
- [x] et l'afficher avec `{:#?}`

---

## Étape 3 — Le lecteur de JSON

*Le gros morceau — 2 à 4 sessions*

**Objectif :** transformer le texte du fichier en `Value`.

**Ce que j'apprends :** `struct` + méthodes, `&mut self`, `match`, et la récursion.

**La structure de base** (à remplir) :

```rust
struct Parser {
    chars: Vec<char>,   // le texte découpé en caractères
    pos: usize,         // où on en est
}
```

Oui, `Vec<char>` est inefficace. C'est volontaire : c'est le plus facile à manipuler, et c'est
précisément ce qu'on optimisera à l'étape 7.

**Dans cet ordre, un sous-objectif à la fois :**

- [x] `null` → comparer des caractères, avancer `pos`
- [x] `true` / `false` → idem, mais deux cas
- [x] les nombres `42`, `-3.14` → accumuler des caractères, `.parse::<f64>()`
- [x] les chaînes `"abc"` → boucle jusqu'au guillemet fermant
- [x] les espaces / retours à la ligne → une méthode `skip_whitespace()`
- [x] les tableaux `[1, 2]` → **la récursion** : lire un tableau = lire des valeurs
- [x] les objets `{"a": 1}` → idem, avec clé + `:`
- [x] les échappements `\n`, `\"`, `\\` → cas particuliers dans les chaînes

**Écrire un test dès le sous-objectif 1.** En bas du fichier :

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lit_null() {
        // assert_eq!(...)
    }
}
```

Puis `cargo test`. Avoir 15 tests qui passent au moment d'attaquer la récursion change complètement
l'expérience : je sais immédiatement si j'ai cassé quelque chose.

**À chercher :** `impl` (attacher des méthodes à un struct), `&mut self`, `match`, `Option`, `if let`.

- [ ] `cargo run -- data.json` affiche l'arbre complet avec `{:#?}`
- [ ] une erreur de syntaxe JSON produit un message d'erreur, pas un plantage

---

## Étape 4 — Réafficher du JSON

*≈ 1-2 h*

**Objectif :** reconvertir le `Value` en texte JSON. D'abord compact, puis indenté.

**Ce que j'apprends :** parcourir récursivement une structure et construire une `String`.

**À chercher :** `impl std::fmt::Display for Value`, la macro `write!`, `String::push_str`.

**Le test qui valide tout le travail depuis le début :**

```bash
cargo run -- data.json > sortie.json
jq . data.json > attendu.json
diff sortie.json attendu.json    # doit être vide
```

- [ ] le `diff` est vide → mon parser est prouvé correct face au vrai `jq`

---

## Étape 5 — Les requêtes

*Le deuxième morceau — 2 à 3 sessions*

**Objectif :** `cargo run -- '.users[0].name' data.json` → `"Alice"`.

**Ce que j'apprends :** le même exercice qu'à l'étape 3 (analyser du texte → structure → l'exécuter),
mais sur un cas plus simple. C'est là que je sens que j'ai compris, parce que ça va vite.

**Découpe en deux :**

- [ ] **Analyser** `.users[0].name` en une liste d'étapes. Un nouvel enum : `Field(String)` ou
      `Index(usize)`. Ici : `[Field("users"), Index(0), Field("name")]`.
- [ ] **Appliquer** : partir de la valeur racine et traverser étape par étape. Une boucle, ou `fold`.

**Puis étendre, une fonctionnalité à la fois :**

- [ ] `.` tout seul (renvoie tout)
- [ ] `.users[]` → itérer sur tous les éléments
- [ ] `length`
- [ ] `select(.admin)`

- [ ] pour 5 requêtes de mon choix, j'obtiens exactement la même sortie que le vrai `jq`

---

## Étape 6 — En faire un vrai outil

*≈ 2 h*

**Objectif :** taper `minijq '.users[].name' data.json` depuis n'importe quel dossier.

**Ce que j'apprends :** l'écosystème Rust et les dépendances externes.

**À chercher :**

- `cargo add clap --features derive` — gestion d'arguments propre, `--help` généré automatiquement
- `cargo add anyhow` — des erreurs lisibles sans effort
- `cargo build --release` puis `cargo install --path .` — installe le binaire dans le PATH

- [ ] j'ai utilisé mon propre outil pour de vrai, au moins une fois, sans y penser

---

## Étape 7 — Le refactor qui apprend Rust pour de bon

**Objectif :** supprimer le `Vec<char>` et arrêter de recopier du texte.

Aujourd'hui, chaque chaîne du JSON est copiée deux fois : une fois dans le `Vec<char>`, une fois dans
le `String` du `Value`. Le but est que le `Value` **pointe directement dans le texte d'origine** au
lieu de le dupliquer.

**Ce que j'apprends :** le borrow checker et les *lifetimes* (durées de vie). Le compilateur va
exiger que je prouve que le texte d'origine vit plus longtemps que les pointeurs qui le désignent —
et il faudra l'écrire explicitement, avec cette syntaxe qui fait peur : `Value<'a>`.

**Se préparer psychologiquement :** cette étape est frustrante, je vais me battre avec le
compilateur. C'est normal, c'est *le* mur de Rust, et tout le monde le prend. La différence avec un
tutoriel : j'ai un besoin concret devant moi, du code qui marche déjà comme point de comparaison, et
des tests pour me dire quand j'ai réussi. C'est de très loin la meilleure situation pour comprendre.

- [ ] `Value` emprunte le texte d'origine au lieu de le copier
- [ ] tous les tests passent toujours
- [ ] j'ai mesuré le gain : générer un gros JSON (quelques Mo) et chronométrer avant/après avec
      `time`. Voir le chiffre baisser rend le concept concret.

---

## Lectures, au fil de l'eau

Ne pas lire le Book en entier avant de commencer — lire **au moment où j'en ai besoin** :

| Quand | Quoi |
|---|---|
| avant l'étape 1 | chapitres 1 à 3 — bases, variables, fonctions |
| avant l'étape 2 | **chapitre 6 — enums et `match`** ⭐ le plus important |
| pendant l'étape 3 | chapitre 5 (structs), chapitre 9 (erreurs) |
| pendant l'étape 7 | **chapitre 4 (ownership) et 10.3 (lifetimes)** — là seulement, ça fera sens |

- The Book en français : https://jimskapt.github.io/rust-book-fr/
- `cargo install rustlings` — petits exercices, parfait pour 20 min d'échauffement
- Documentation d'une bibliothèque : https://docs.rs/nom-du-crate

---

## Réflexes venant de Java / TypeScript à désapprendre

- **L'héritage n'existe pas.** Le réflexe « classe abstraite + 3 sous-classes » devient dans 80 % des
  cas un `enum` avec 3 variants, pas un `Box<dyn Trait>`. Les traits servent à l'abstraction *ouverte*
  (des tiers implémentent), les enums à l'abstraction *fermée* (je connais tous les cas). Java n'a que
  le premier outil, d'où l'abus.
- **Tout n'est pas sur le tas.** En Java chaque objet est une référence vers le heap. En Rust c'est la
  pile par défaut, et les valeurs se *déplacent*. C'est le vrai choc, plus que le borrow checker.
- **Le piège `Rc<RefCell<T>>`.** Modéliser un graphe d'objets qui se pointent mutuellement, comme en
  Java, va faire souffrir. On trouve `Rc<RefCell<T>>` sur Stack Overflow, et on écrit du Java déguisé
  qui plante à l'exécution. La bonne réponse quasi systématique : un `Vec<Node>` et des `usize` comme
  identifiants.
- **Immutable par défaut**, l'inverse de Java (`final` partout implicitement).

---

## Ce que ce projet n'apprend pas

Le parallélisme et l'async. C'est volontaire : ce sont les sujets les plus casse-gueule de Rust, à
garder pour le projet n°2, une fois les bases acquises.

**Idées pour la suite :** un analyseur de logs performant (parallélisme avec `rayon`), un détecteur
d'exports morts pour un monorepo TS basé sur le parser [`oxc`](https://oxc.rs/), un émulateur CHIP-8.
