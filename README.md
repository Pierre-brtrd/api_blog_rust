# Blog API RUST

[![Rust](https://img.shields.io/badge/language-Rust-000000?logo=rust)](https://www.rust-lang.org/) [![API](https://img.shields.io/badge/API-REST-blue?logo=rest)](https://en.wikipedia.org/wiki/Representational_state_transfer) [![DDD](https://img.shields.io/badge/architecture-DDD-green)](https://en.wikipedia.org/wiki/Domain-driven_design)

Une API REST en **Rust**, bâtie avec **Actix-Web**, **SQLite** (via SQLx) et un design **DDD** (Domain-Driven Design).  
Elle gère les **utilisateurs** (avec rôles et authentification JWT) et les **articles** (CRUD), le tout organisé en couches claires.

---

## 🚀 Fonctionnalités principales

-   **Auth & JWT** : inscription, login, génération et validation de tokens JWT
-   **Gestion des utilisateurs** (`/users`)
    -   CRUD (create, read, update, delete)
    -   Sécurisé : accès restreint aux rôles **Admin**
-   **Gestion des posts** (`/posts`)
    -   CRUD complet
    -   Protégé par JWT (tous les utilisateurs authentifiés)
-   **Migrations SQLx** : création et mise à jour de la base SQLite
-   **TLS/HTTPS** avec OpenSSL
-   **Validation** des données entrantes (`validator` + DTO)

---

## 📦 Prérequis

-   **Rust** (cargo) ≥ 1.68
-   **sqlx-cli** (pour migrations) :
    ```bash
    cargo install sqlx-cli --no-default-features --features sqlite
    ```

*   **OpenSSL** (pour TLS)
*   **mkcert** ou équivalent (génération de certificats locaux)
*   Copiez `.env.template` → `.env` et remplissez vos variables (voir ci-dessous)

---

## ⚙️ Configuration

1. **Dupliquez** le template et ajustez :

    ```bash
    cp .env.template .env
    ```

2. **Editez** `.env` :

    ```dotenv
    DATABASE_URL=sqlite://blog_rust.db
    SERVER__HOST=127.0.0.1
    SERVER__PORT=8443

    # TLS
    TLS__CERT_PATH=./certs/localhost.pem
    TLS__KEY_PATH=./certs/localhost-key.pem

    # JWT
    JWT_SECRET=votre_cle_très_secrète
    ```

3. (Optionnel) **Générez** un certificat local :

    ```bash
    mkcert -install
    mkcert localhost 127.0.0.1 ::1
    ```

---

## 💾 Base de données & migrations

```bash
# 1) Crée (si nécessaire) le fichier SQLite
sqlx db create

# 2) Applique les migrations (création tables users & posts, rôle…)
sqlx migrate run
```

> À chaque ajout/modification de migration :
>
> ```bash
> sqlx migrate run
> ```

---

## 🏃 Démarrage

```bash
cargo run --release
```

-   L’API écoute en HTTPS sur `https://{SERVER__HOST}:{SERVER__PORT}`
-   Les logs SQLx et Actix sont activés (niveau DEBUG si souhaité via `RUST_LOG=debug`)

---

## 🔗 Endpoints

| Méthode | Chemin        | Auth       | Rôle requis | Description                  |
| :------ | :------------ | :--------- | :---------: | :--------------------------- |
| POST    | `/login`      | Aucune     |      —      | Authentification (JWT)       |
| POST    | `/users`      | Bearer JWT |    Admin    | Créer un utilisateur         |
| GET     | `/users`      | Bearer JWT |    Admin    | Lister tous les utilisateurs |
| GET     | `/users/{id}` | Bearer JWT |    Admin    | Récupérer un utilisateur     |
| PATCH   | `/users/{id}` | Bearer JWT |    Admin    | Mettre à jour un utilisateur |
| DELETE  | `/users/{id}` | Bearer JWT |    Admin    | Supprimer un utilisateur     |
| GET     | `/posts`      | Bearer JWT | Authentifié | Lister tous les posts        |
| POST    | `/posts`      | Bearer JWT | Authentifié | Créer un post                |
| GET     | `/posts/{id}` | Bearer JWT | Authentifié | Récupérer un post            |
| PATCH   | `/posts/{id}` | Bearer JWT | Authentifié | Mettre à jour un post        |
| DELETE  | `/posts/{id}` | Bearer JWT | Authentifié | Supprimer un post            |

> 📘 Tous les endpoints **/users** sont doublés d’un middleware **Admin**.
> 📘 Tous les endpoints **/posts** requièrent un JWT valide.

---

## 🏗️ Architecture DDD

```
src/
├── domain/         # Entités métier, erreurs, validation, traits de repo
│   ├── model/
│   │   ├── user.rs
│   │   └── post.rs
│   ├── repository.rs
│   ├── error.rs
│   └── validation.rs
│
├── application/    # Logique métier (services)
│   ├── user_service.rs
│   └── post_service.rs
│
├── infrastructure/ # Implémentations techniques
│   ├── db/ (migrations + init_db)
│   ├── keys.rs
│   ├── tls.rs
│   ├── auth/ (JWT, admin middleware, hashing)
│   └── persistence/sqlite/ (repos SQLx)
│
└── interfaces/     # Couche web (DTO, handlers, routes, erreurs API)
    ├── api/
    │   ├── dto/         # Request/Response structs + validation
    │   ├── handlers/    # Actix-Web handlers & routes
    │   ├── error.rs     # Mapping DomainError → ApiError
    │   └── validation.rs
    └── config/         # Chargement `.env` → Settings
```

-   **Domain** : votre cœur métier et ses invariants
-   **Application** : use-cases, orchestration de repos et validation
-   **Infrastructure** : communication BDD, token, TLS, hashing…
-   **Interfaces** : adaptateurs HTTP (Actix-Web), DTO/validations, routage

---

## 🔧 Commandes utiles

-   **Formatter** : `cargo fmt`
-   **Lint** : `cargo clippy`
-   **Tests** : `cargo test`
-   **Migrations** :

    -   Ajouter une migration : `sqlx migrate add -r <nom_migration>`
    -   Voir l’état : `sqlx migrate info`
    -   Appliquer : `sqlx migrate run`

---

## 📚 Ressources

-   **Actix-Web** : [https://actix.rs](https://actix.rs)
-   **SQLx** : [https://docs.rs/sqlx](https://docs.rs/sqlx)
-   **Serde + validator** : [https://crates.io/crates/validator](https://crates.io/crates/validator)
-   **jsonwebtoken (JWT)** : [https://crates.io/crates/jsonwebtoken](https://crates.io/crates/jsonwebtoken)

---

> Crafted with ❤️ in Rust, **work-in-progress** 🚧
