use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, available_parallelism, JoinHandle};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

use crate::server::handle_client; 

pub type Work = (TcpStream, SocketAddr); 


pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Sender<Work>
}

pub struct Worker {
    id: usize,
    handle: JoinHandle<()> ,
}


impl ThreadPool {
    async fn new() -> Self {
        let _av = available_parallelism().unwrap();
        let av = usize::from(_av);
        let av = av - 1usize;
        let (tx, rx) = channel();
        let mut workers: Vec<Worker> = Vec::with_capacity(av); 
        let inner_rx = Arc::new(Mutex::new(rx));

        for i in 0..av {
            let worker = Worker::new(inner_rx.clone(), i).await;
            workers.push(worker);
            
        }

        ThreadPool {
            workers,
            tx
        }

    }
}



impl Worker {
    async fn new(
    rx: Arc<Mutex<Receiver<Work>>>,
    id: usize
) -> Self {
        let handle = thread::spawn(move ||{
            loop{
                while let Ok(work) = rx.lock().unwrap().recv()
                {
                    
                    
                }             
            }
        });
        Worker {id, handle}
    }
}






