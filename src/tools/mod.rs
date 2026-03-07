pub mod tools {
    use actix_web::HttpResponse;
    use crate::logger::logger::LogLevel;
    use crate::LOGGER;
    use tokio_postgres::tls::NoTlsStream;
    use tokio_postgres::{Connection, Socket};

    pub fn spawn_connection_thread(
        connection: Connection<Socket, NoTlsStream>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                LOGGER.log(&format!("connection error: {}", e), LogLevel::CRITICAL);
            }
        })
    }
}
