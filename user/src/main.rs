use snafu::ResultExt;
use user::{config::UserConfig, server::get_server};

#[actix_web::main]
async fn main() -> Result<(), snafu::Whatever>{
    let user_config = UserConfig::get();
    let server = get_server(&user_config)?;

    server.await.whatever_context("Server stopped running")?;
    
    Ok(())
}
