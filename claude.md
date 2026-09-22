# CLAUDE.md

## Projet

Application de bureau (Tauri + Svelte 5) permettant à des participants de voter pour leurs artistes préférés (top 3), avec une partie admin protégée par mot de passe pour consulter le classement et tirer un votant au sort.

Langue de l'interface : **français**. Code, commentaires et noms de variables : **anglais**.

## Initialisation

Le projet est **déjà créé** avec `npm create tauri-app@latest` (Tauri 2, template Svelte + TypeScript). Ne pas recréer ni réorganiser le scaffolding : travailler à partir de la structure existante et l'étendre.

## Stack

- **Shell desktop** : Tauri 2
- **Frontend** : Svelte 5 (runes) + Vite, en SPA (pas de SSR), tel que généré par le template Tauri
- **Langage** : TypeScript strict
- **Styles** : SCSS (variables centralisées dans `src/lib/styles/_variables.scss`)
- **Stockage** : SQLite local via `tauri-plugin-sql` (ou `rusqlite` côté Rust si la logique métier y est déplacée)
- **Package manager** : npm

## Commandes

```bash
npm install           # installer les dépendances
npm run tauri dev     # lancer l'app en dev
npm run tauri build   # build de production
npm run check         # svelte-check + tsc
npm run lint          # lint (à configurer si absent)
```

## Mises à jour de l'application

L'app intègre un auto-updater (`tauri-plugin-updater` + `tauri-plugin-process`) : un bouton « Vérifier les mises à jour » dans l'admin (`src/lib/components/admin/UpdateChecker.svelte`) télécharge et installe la dernière version publiée sur GitHub Releases, puis relance l'app — sur Mac comme sur Windows.

**Publier une nouvelle version** :
1. Bumper `"version"` dans `src-tauri/tauri.conf.json`, `package.json` **et** `version` dans `src-tauri/Cargo.toml` (même valeur partout — c'est la version dans `tauri.conf.json` qui est réellement comparée par l'updater, mais les trois doivent rester synchronisés).
2. Committer, puis taguer : `git tag vX.Y.Z && git push origin vX.Y.Z`.
3. Le workflow `.github/workflows/release.yml` construit automatiquement les installeurs macOS (universal) et Windows, les signe, et publie une GitHub Release avec le `latest.json` que l'updater consulte. Rien d'autre à faire.

**Clé de signature** : la paire de clés vit hors du repo (`~/.tauri/expodesartistes.key` + `.key.pub`, jamais commitées). La clé privée et son mot de passe sont enregistrés comme secrets GitHub Actions (`TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`). La clé publique est dans `src-tauri/tauri.conf.json` (`plugins.updater.pubkey`) — elle peut être commitée sans risque. **Si la clé privée est perdue, les mises à jour futures ne pourront plus être installées automatiquement par les utilisateurs existants** (il faudra republier une nouvelle clé et redistribuer l'app manuellement une fois).

## Fonctionnalités

### 1. Vote (côté votant)

Formulaire en un seul écran :

| Champ | Règles |
|---|---|
| Prénom | Requis, trim, 2 à 50 caractères |
| Nom | Requis, trim, 2 à 50 caractères |
| Téléphone | Requis, numéro valide (format international ou belge), normalisé en E.164 avant stockage |
| Top 1 / Top 2 / Top 3 | Requis, 3 artistes **distincts** |

**Sélection du top 3**
- Trois champs cliquables : « Top 1 », « Top 2 », « Top 3 ».
- Un clic ouvre un **modal** listant les artistes participants avec leur numéro attribué, ex. `#1 Léonard de Vinci`.
- Un artiste déjà choisi dans un autre rang est grisé/désactivé dans le modal (ou remplacé avec confirmation).
- Après sélection, le champ affiche `#numéro Nom` et permet de rechanger le choix.

**Confirmation**
- Le bouton « Confirmer mon vote » n'est actif que si tous les champs sont valides.
- Au clic : enregistrement du vote, écran de confirmation, puis réinitialisation du formulaire pour le votant suivant.

**Unicité des votants** (deux contraintes indépendantes, message d'erreur clair pour chacune) :
- **Téléphone** : un seul vote par numéro (normalisé E.164).
- **Prénom + nom** : la **combinaison** doit être unique. « Jean Dupont » ne peut voter qu'une fois, mais « Jean Dupont » et « Marc Dupont » sont autorisés. La comparaison ignore la casse, les accents et les espaces superflus (« jean  DUPONT » = « Jean Dupont »).

### 2. Système de points

| Rang | Points |
|---|---|
| Top 1 | 3 |
| Top 2 | 2 |
| Top 3 | 1 |

Le score d'un artiste = somme des points reçus. Départage en cas d'égalité : nombre de Top 1, puis de Top 2, puis ordre par numéro d'artiste. Les constantes de points vivent dans un seul fichier (`src/lib/config/points.ts`).

### 3. Admin

- Accès via un mot de passe (route `/admin`), demandé à chaque ouverture de session admin.
- **Onglet Classement** : liste triée des artistes avec rang, numéro, nom, points totaux, et détail (nb de Top 1 / Top 2 / Top 3). Nombre total de votes affiché.
- **Onglet Artistes** : liste des artistes existants et formulaire d'ajout avec **nom d'artiste** et **numéro**. Le numéro est prérempli avec `dernier numéro + 1` (ou `1` si la liste est vide) et reste modifiable. Le numéro doit être un entier positif unique ; le nom est requis.
  - **Modification** : le nom et le numéro d'un artiste sont modifiables (mêmes règles de validation, le nouveau numéro ne doit pas déjà être utilisé). Les votes existants suivent automatiquement le nouveau numéro.
  - **Suppression** : avec une confirmation (« Supprimer l'artiste #N Nom ? »). **Refusée si l'artiste a déjà reçu au moins un vote**, avec un message explicite indiquant le nombre de votes concernés. Suppression possible uniquement pour un artiste sans vote.
  - Le nombre de votes par artiste est affiché dans la liste pour anticiper ce blocage.
- **Onglet Tirage au sort** : bouton qui sélectionne **un votant au hasard** parmi tous les votants et affiche prénom, nom et téléphone (pour l'appeler et lui remettre un prix). Le tirage n'a lieu qu'une fois en pratique ; un bouton « Tirer à nouveau » (qui exclut les votants déjà tirés durant la session) reste disponible au cas où le gagnant est absent.
- Bouton de déconnexion admin.

**Mot de passe admin**
- Ne jamais le coder en dur dans le frontend.
- Stocké sous forme de hash (Argon2) côté Rust, vérifié via une commande Tauri (`invoke`). Le hash est configuré au premier lancement ou via une variable d'environnement au build.

## Modèle de données

```sql
artists (
  id          INTEGER PRIMARY KEY,   -- numéro attribué (#1, #2, ...), saisi et modifiable par l'admin
  name        TEXT NOT NULL
);

voters (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  first_name  TEXT NOT NULL,
  last_name   TEXT NOT NULL,
  phone       TEXT NOT NULL UNIQUE,  -- E.164
  name_key    TEXT NOT NULL UNIQUE,  -- prénom|nom normalisés (minuscules, sans accents, espaces réduits)
  top1_id     INTEGER NOT NULL REFERENCES artists(id) ON UPDATE CASCADE ON DELETE RESTRICT,
  top2_id     INTEGER NOT NULL REFERENCES artists(id) ON UPDATE CASCADE ON DELETE RESTRICT,
  top3_id     INTEGER NOT NULL REFERENCES artists(id) ON UPDATE CASCADE ON DELETE RESTRICT,
  created_at  TEXT NOT NULL DEFAULT (datetime('now')),
  CHECK (top1_id <> top2_id AND top1_id <> top3_id AND top2_id <> top3_id)
);
```

La table `artists` démarre vide et est alimentée depuis l'onglet admin. Le vote est impossible tant qu'il y a moins de 3 artistes (message explicite à l'écran).
`name_key` est calculé **côté Rust** avant insertion ; la contrainte `UNIQUE` en base fait foi.
Activer `PRAGMA foreign_keys = ON` à chaque connexion SQLite (désactivé par défaut) : c'est ce qui rend effectifs le `ON UPDATE CASCADE` (changement de numéro) et le `ON DELETE RESTRICT` (suppression bloquée si l'artiste a des votes).

## Architecture

```
src/
├── routes/            # écrans (vote, confirmation, admin)
├── lib/
│   ├── components/    # composants UI réutilisables (Modal, ArtistPicker, Button, Input...)
│   ├── stores/        # états partagés (runes : $state / $derived dans .svelte.ts)
│   ├── services/      # accès aux commandes Tauri (invoke), aucune logique UI
│   ├── config/        # constantes (points, règles de validation)
│   ├── utils/         # validation, normalisation téléphone
│   └── styles/        # SCSS global et variables
src-tauri/
└── src/               # commandes Rust (vote, artistes, classement, tirage, auth admin)
```

- Logique métier sensible (validation finale, calcul du classement, tirage, auth admin) **côté Rust** ; le frontend ne fait que de l'affichage et de la validation d'UX.
- Validation dupliquée : côté frontend pour le retour immédiat, côté Rust pour la garantie.

## Conventions Svelte 5

- Utiliser exclusivement les **runes** (`$state`, `$derived`, `$effect`, `$props`). Pas de `export let`, pas de `$:`, pas de stores legacy (`writable`) sauf nécessité.
- Snippets (`{#snippet}` / `{@render}`) à la place des slots.
- Événements via attributs (`onclick`, pas `on:click`).
- Composants petits et typés ; props explicitement typées via `$props()`.
- Éviter `$effect` quand un `$derived` suffit.

## UI / Design

- **Thème clair uniquement**, fond de page **`#FAF8F5`** (quasi blanc ; les cartes se distinguent par une ombre douce plutôt qu'un contraste de fond — palette complète dans `src/lib/styles/_variables.scss`).
- Design **simple, épuré et accessible** : priorité à la lisibilité et à la facilité d'usage, y compris pour des personnes peu à l'aise avec l'informatique.
- Grandes zones cliquables (min. 44×44 px), texte ≥ 16 px, contrastes conformes WCAG AA minimum.
- Modal : focus piégé, fermeture avec `Échap`, `role="dialog"` + `aria-modal="true"`, liste navigable au clavier.
- Champs avec `<label>` explicite, messages d'erreur clairs en français, placés sous le champ concerné et liés via `aria-describedby`.
- États visibles : focus, hover, désactivé, erreur, succès.
- Une seule action principale par écran.
- Aucune dépendance UI lourde sans raison ; privilégier des composants maison ou Bits UI pour les primitives accessibles (dialog).

## Règles de travail

- Rester **concis** : ne pas ajouter de fonctionnalités non demandées (pas de comptes utilisateurs, pas d'export, pas d'animations superflues) sans validation.
- Ne pas modifier le barème de points ni les règles de vote sans demande explicite.
- Commits en français ou anglais, format conventionnel (`feat:`, `fix:`, `refactor:`), branches selon Git Flow.
- Avant de terminer une tâche : `npm run check` (et le lint s'il est configuré) doit passer.
- En cas d'ambiguïté fonctionnelle, poser la question plutôt que supposer.