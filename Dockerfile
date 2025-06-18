# Étape de build Rust (inchangée)
FROM rust:latest AS builder
WORKDIR /usr/src/app

# ... cache des dépendances ...
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo fetch

COPY . .
RUN cargo build --release

# Étape finale
FROM debian:bookworm-slim
WORKDIR /app

# Installer openssl pour la génération de certs
RUN apt-get update && \
    apt-get install -y --no-install-recommends openssl && \
    rm -rf /var/lib/apt/lists/*

# Copier binaire et entrypoint
COPY --from=builder /usr/src/app/target/release/blog-api /app/blog-api
# Copie du script d’entrypoint depuis le contexte de build
ADD entrypoint.sh /app/entrypoint.sh

# On donne les droits d’exécution
RUN chmod +x /app/entrypoint.sh

# Création de dossier de certs
RUN mkdir -p /app/certs

# Port exposé
EXPOSE 443

# Pas de Server header par défaut
ENV SERVER__HOST=127.0.0.1
ENV SERVER__PORT=443

# Entrypoint
ENTRYPOINT ["/app/entrypoint.sh"]
