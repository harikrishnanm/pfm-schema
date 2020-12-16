use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use log::{debug, info};
use num_cpus;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager, PoolError };
use core::time::Duration;


pub type PgPool = Pool<ConnectionManager<PgConnection>>;


pub fn get_server_address() -> SocketAddr {
    let default_port = 8088;
    let address = IpAddr::from_str("127.0.0.1").unwrap();

    let mut port: u16 = default_port;

    match env::var("SERVER_PORT") {
        Ok(val) => {
            port = u16::from_str(&val).unwrap();
            info!("Setting port to {}", port);
        }
        Err(_e) => info!("No SERVER_PORT environment variable set. Using default {}", port)
    };
    debug!("Socket address: {:?} port {}", address, port);
    SocketAddr::new(address, port)
}

pub fn get_worker_count() -> usize {
    let mut workers = num_cpus::get();
    match env::var("WORKERS") {
        Ok(val) => {
            workers = usize::from_str(&val).unwrap();
            info!("Setting worker count to {}", workers);
        }
        Err(_e) => info!("No WORKERS environment variable. Using default {}", workers)
    };
    workers
}

pub(crate) async fn get_db() -> Result<PgPool, PoolError> {

    let manager = ConnectionManager::<PgConnection>::new(dotenv!("DATABASE_URL"));

    Pool::builder()
        .max_size(15)
        .min_idle(Some(3))
        .connection_timeout(Duration::new(5,0))
        .idle_timeout(Some(Duration::new(60, 0)))
        .build(manager)
}