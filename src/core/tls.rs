use anyhow::Result;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslMethod};

pub fn build_ssl_acceptor(cert_path: &str, key_path: &str) -> Result<SslAcceptorBuilder> {
    let mut builder: SslAcceptorBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    builder.set_private_key_file(key_path, openssl::ssl::SslFiletype::PEM)?;

    builder.set_certificate_chain_file(cert_path)?;

    Ok(builder)
}
