use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::ops::Deref;

use postgres::Connection;
use postgres::TlsMode;

use crate::error::{Error, Result};

pub static DEFAULT_POOL_SIZE: usize = 5;

#[derive(Clone)]
pub struct ConnectionPool {
    host: String,
    inner: Arc<Mutex<Pool>>,
    wait_lock: Arc<Condvar>
}

struct Pool {
    pub size: usize,
    pub len: Arc<AtomicUsize>,
    conns: Vec<Connection>
}

pub struct PooledConn {
    conn: Option<Connection>,
    pool: Arc<Mutex<Pool>>,
    wait_lock: Arc<Condvar>
}

impl PooledConn {
    pub fn get_conn(&self) -> &Connection {
        self.conn.as_ref().unwrap()
    }
}

impl Drop for PooledConn {
    fn drop(&mut self) {
        if let Ok(mut locked) = self.pool.lock() {
            locked.conns.push(self.conn.take().unwrap());
            self.wait_lock.notify_one();
        }
    }
}

impl Deref for PooledConn {
    type Target = Connection;

    #[inline]
    fn deref(&self) -> &Connection {
        self.conn.as_ref().unwrap()
    }
}

impl ConnectionPool {
    pub fn new(host: &str) -> ConnectionPool {
        ConnectionPool::with_size(host, DEFAULT_POOL_SIZE)
    }

    pub fn with_size(host: &str, size: usize) -> ConnectionPool {
        ConnectionPool {
            host: host.to_owned(),
            wait_lock: Arc::new(Condvar::new()),
            inner: Arc::new(Mutex::new(Pool {
                len: Arc::new(ATOMIC_USIZE_INIT),
                size: size,
                conns: Vec::with_capacity(size)
            }))
        }
    }

    pub fn set_size(&self, size: usize) -> Result<()> {
        if size < 1 {
            Err(Error::Message("The connection pool size must be greater than zero.".to_string()))
        } else {
            let mut locked = self.inner.lock().unwrap();
            locked.size = size;
            Ok(())
        }
    }

    pub fn acquire_conn(&self) -> Result<PooledConn> {
        let mut locked = self.inner.lock().unwrap();
        if locked.size == 0 {
            return Err(Error::Message("The connection pool does not allow \
                                                    connections; increase the size of the pool.".to_string()))
        }

        loop {
            if let Some(conn) = locked.conns.pop() {
                return Ok(PooledConn {
                    conn: Some(conn),
                    pool: self.inner.clone(),
                    wait_lock: self.wait_lock.clone()
                })
            }

            let len = locked.len.load(Ordering::SeqCst);
            if len < locked.size {
                let conn = Connection::connect(self.host.clone(), TlsMode::None).unwrap();
                let _ = locked.len.fetch_add(1, Ordering::SeqCst);
                return Ok(PooledConn {
                    conn: Some(conn),
                    pool: self.inner.clone(),
                    wait_lock: self.wait_lock.clone()
                });
            }

            locked = self.wait_lock.wait(locked).unwrap();
        }
    }
}

pub fn aaa() {
    let pool = ConnectionPool::new("postgresql://postgres:123456@localhost");

    let pool = pool.clone();

    let conn = pool.acquire_conn().unwrap();

    conn.execute("", &[]);
}

    // let pool = util::posrgres_pool::ConnectionPool::new("postgresql://postgres:123456@localhost");

    // let mut threads = Vec::new();

    // for _ in 0..1000 {
    //     let pool = pool.clone();

    //     threads.push(::std::thread::spawn(move || {
    //         let polled_conn = pool.acquire_conn().unwrap();

    //         let conn = polled_conn.get_conn();

    //         // println!("{:?}", rows);

    //         // conn.execute("CREATE TABLE person (
    //         //         id              SERIAL PRIMARY KEY,
    //         //         name            VARCHAR NOT NULL,
    //         //         data            BYTEA
    //         //       )", &[]).unwrap();
    //         // let me = Person {
    //         //     id: 0,
    //         //     name: "Steven".to_string(),
    //         //     data: None,
    //         // };
    //         //conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
    //                      //&[&me.name, &me.data]).unwrap();
    //         for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
    //             let person = Person {
    //                 id: row.get(0),
    //                 name: row.get(1),
    //                 data: row.get(2),
    //             };
    //             //println!("Found person {}", person.name);
    //         }

    //         ::std::thread::sleep_ms(500);
    //     }));
    // }

    // for t in threads {
    //     t.join();
    // }
