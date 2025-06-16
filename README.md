# Blog API (Rust + Actix-Web + SQLite)

Une API REST pour gérer un blog avec utilisateurs, authentification JWT, article, et persistance SQLite via SQLx.

---

## 📦 Prérequis

-   Rust (cargo) ≥ 1.68
-   `sqlx-cli` pour gérer les migrations :
    ```bash
    cargo install sqlx-cli --no-default-features --features sqlite
    ```
-   OpenSSL (pour HTTPS avec `bind_openssl`)
-   Un fichier `.env` à la racine du projet (voir ci-dessous)

---

## ⚙️ Configuration

1. Dupliquer `.env.template` en `.env` et remplir les valeurs :

    ```dotenv
    DATABASE_URL=sqlite://blog_rust.db
    SERVER_HOST=127.0.0.1
    SERVER_PORT=8443

    # TLS (OpenSSL)
    TLS_CERT_PATH=./certs/localhost+2.pem
    TLS_KEY_PATH=./certs/localhost+2-key.pem

    # Auth JWT
    JWT_SECRET=une_cle_très_longue_et_secrète
    ```

2. (Optionnel) Générer les certificats locaux avec mkcert ou OpenSSL :

    ```bash
    mkcert -install
    mkcert localhost 127.0.0.1 ::1
    ```

---

## 🚀 Initialisation de la base de données

Avant de lancer l’appli, exécutez **deux** commandes pour :

1. créer le fichier SQLite
2. appliquer les migrations SQLx

```bash
# 1) Crée (si manquant) la base SQLite et le fichier `blog_rust.db`
sqlx db create

# 2) Exécute toutes les migrations (création des tables users & posts)
sqlx migrate run
```

> **Remarque**
> Si à l’avenir vous ajoutez ou modifiez une migration, il suffira de relancer :
>
> ```bash
> sqlx migrate run
> ```

---

## 🏃‍♂️ Démarrage de l’API

```bash
cargo run --release
```

-   L’API écoute en HTTPS sur `https://SERVER_HOST:SERVER_PORT` (par défaut `8443`).
-   Points d’entrée principaux :
    -   **`POST /users/register`** — création d’un utilisateur
    -   **`POST /users/login`** — authentification (retourne un JWT)
    -   **`GET|POST /posts`** — gestion des billets (JWT requis)

---

## 🗂️ Structure du projet

```
.
├── Cargo.toml
├── .env.example
├── migrations/               # SQLx migrations (users + posts)
│   ├── …up.sql
│   └── …down.sql
├── certs/                    # certificats TLS (mkcert / OpenSSL)
│   ├── localhost+2.pem
│   └── localhost+2-key.pem
├── src/
│   ├── main.rs               # point d’entrée : config + init_db + serveur
│   ├── api/                  # handlers, routes, DTO de réponse
│   ├── core/
│   │   ├── db.rs             # init_db + migrations
│   │   ├── tls.rs            # build_ssl_acceptor (OpenSSL)
│   │   └── server.rs         # AppState + montage de l’App
│   ├── config/               # Settings (config-rs / dotenv)
│   ├── domain/               # entités, DTO, erreurs, traits repo
│   └── infra/                # implémentations des repository SQLite
└── README.md
```

---

## 🔧 Commandes utiles

-   **Formatter** : `cargo fmt`
-   **Lint** : `cargo clippy`
-   **Tests** : `cargo test`
-   **Migrations** :

    -   ajouter une migration :

        ```bash
        sqlx migrate add -r ma_nouvelle_migration
        ```

    -   voir l’état : `sqlx migrate info`
    -   appliquer : `sqlx migrate run`

---

## 🧪 Tests

Un dossier API Blog - RUST est à disposition pour tester sur BRUNO les différents endpoints de l'API.

---

## 📖 Documentation

-   [Actix-Web](https://actix.rs)
-   [SQLx](https://docs.rs/sqlx)
-   [Serde + validator](https://crates.io/crates/validator)
-   [jsonwebtoken (JWT)](https://crates.io/crates/jsonwebtoken)

---

> **Blog API** — Développé en Rust, work in progress 🚧
