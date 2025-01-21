use std::io::Read;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use lib_db::types::PgPool;
use log::info;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use tokio::net::TcpStream;


type Job = (TcpStream, SocketAddr);


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>

}

static mut TH: u8 = 0;

impl ThreadPool {
    pub fn new(size: usize, pool: PgPool) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();

        let prim_recevier = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&prim_recevier), pool.clone()));
        }


        ThreadPool {workers, sender}
    }
    pub fn execute(&self, f: Job)
    {
        self.sender.send(f).unwrap();
    }
}



struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}


async fn handle(st: (TcpStream, SocketAddr), pool: &PgPool) {
    let _p = pool;
    let mut buf = String::new();
    let mut stream = st.0;
    let addr = st.1;
    info!("client: {addr} is being served");
    stream.read_to_string(&mut buf).await.unwrap();
    let mut f = std::fs::File::open("../../../res.html").unwrap();
    let mut buff = vec![0; 4000];
    let file_size = f.read_to_end(&mut buff).unwrap();
    stream.write_all(&buff).await.unwrap();
    println!("file_size: {file_size}");

}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
        pool: PgPool
    ) -> Worker {
       let thread = thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap().block_on(async move { 
                while let Ok(job) = receiver.lock().unwrap().recv()  {
                    println!("thread {id}");
                    handle(job, &pool).await
                }       
            })
        }); 

        Worker { id, thread }
    }
}



