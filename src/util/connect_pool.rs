use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::ops::Deref;

use error::{Error, Result};

pub static DEFAULT_POOL_SIZE: usize = 5;

pub type Handle<T> = Fn() -> Result<T> + Send + 'static;

pub struct ConnectPool<T> {
    inner: Arc<Mutex<Pool<T>>>,
    wait_lock: Arc<Condvar>,
    connect_fn: Arc<Box<Handle<T>>>
}

struct Pool<T> {
    pub size: usize,
    pub len: Arc<AtomicUsize>,
    conns: Vec<T>
}

pub struct PooledConn<T> {
    conn: Option<T>,
    pool: Arc<Mutex<Pool<T>>>,
    wait_lock: Arc<Condvar>
}

impl<T> ConnectPool<T> {
    pub fn new<F>(connect_fn: F) -> ConnectPool<T>
        where F: Fn() -> Result<T> + Send + 'static
    {
        ConnectPool::with_size(connect_fn, DEFAULT_POOL_SIZE)
    }

    pub fn with_size<F>(connect_fn: F, size: usize) -> ConnectPool<T>
        where F: Fn() -> Result<T> + Send + 'static
    {
        ConnectPool {
            wait_lock: Arc::new(Condvar::new()),
            inner: Arc::new(Mutex::new(Pool {
                len: Arc::new(ATOMIC_USIZE_INIT),
                size: size,
                conns: Vec::with_capacity(size)
            })),
            connect_fn: Arc::new(Box::new(connect_fn))
        }
    }

    pub fn acquire_conn(&self) -> Result<PooledConn<T>> {
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

                let conn = (self.connect_fn)()?;

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

impl<T> Clone for ConnectPool<T> {
    fn clone(&self) -> ConnectPool<T> {
        ConnectPool {
            inner: self.inner.clone(),
            wait_lock: self.wait_lock.clone(),
            connect_fn: self.connect_fn.clone()
        }
    }
}

impl<T> Drop for PooledConn<T> {
    fn drop(&mut self) {
        if let Ok(mut locked) = self.pool.lock() {
            locked.conns.push(self.conn.take().unwrap());
            self.wait_lock.notify_one();
        }
    }
}

impl<T> Deref for PooledConn<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.conn.as_ref().unwrap()
    }
}



    // let conn_pool = ConnectPool::with_size(|| {
    //     Ok(Connection::connect("127.0.0.1:5433", TlsMode::None).unwrap())
    // }, 5);

    // let pool = conn_pool.clone();

    // let conn = pool.acquire_conn().unwrap();

    // conn.execute("SELECT * FROM table", &[]);
