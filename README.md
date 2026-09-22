# Exposition des artistes

Application de bureau (Tauri 2 + Svelte 5) pour voter pour ses artistes préférés (top 3), avec une partie admin protégée par mot de passe pour consulter le classement et tirer un votant au sort.

## Commandes

```bash
npm install           # installer les dépendances
npm run tauri dev     # lancer l'app en dev
npm run tauri build   # build de production
npm run check         # svelte-check + tsc
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Publier une nouvelle version

L'app intègre un auto-updater : un bouton dans l'onglet **Version** de l'admin télécharge et installe la dernière version publiée sur GitHub Releases, puis relance l'app (Mac comme Windows).

1. Bumper le numéro de version aux **trois** endroits (même valeur partout) :
   - `"version"` dans `src-tauri/tauri.conf.json` (c'est celle-ci que l'updater compare réellement)
   - `"version"` dans `package.json`
   - `version` dans `src-tauri/Cargo.toml`
2. Committer le bump de version :
   ```bash
   git add src-tauri/tauri.conf.json package.json src-tauri/Cargo.toml
   git commit -m "chore: bump version to X.Y.Z"
   ```
3. Créer le tag et le pousser :
   ```bash
   git tag vX.Y.Z
   git push origin main
   git push origin vX.Y.Z
   ```
4. Le workflow `.github/workflows/release.yml` construit automatiquement les installeurs macOS (universal) et Windows, les signe, et publie une GitHub Release avec le `latest.json` consulté par l'updater. Rien d'autre à faire.

**Clé de signature** : la paire de clés vit hors du repo (`~/.tauri/expodesartistes.key` + `.key.pub`, jamais commitées). La clé privée et son mot de passe sont enregistrés comme secrets GitHub Actions (`TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`). La clé publique est dans `src-tauri/tauri.conf.json` (`plugins.updater.pubkey`) et peut être commitée sans risque. Si la clé privée est perdue, les mises à jour futures ne pourront plus être installées automatiquement par les utilisateurs existants.
