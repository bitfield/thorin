use rand::Rng;
use std::time::Duration;
use tokio::sync::mpsc;

struct Object {
    tx: mpsc::Sender<String>,
    msg: String,
}

async fn run(obj: Object) {
    loop {
        let delay = rand::thread_rng().gen_range(1..5);
        tokio::time::sleep(Duration::from_secs(delay)).await;
        obj.tx.send(obj.msg.clone()).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);
    let tx2 = tx.clone();
    let thorin = Object {
        tx,
        msg: "Thorin sits down and starts singing about gold".into(),
    };
    let gandalf = Object {
        tx: tx2,
        msg: r#"Gandalf says "What's this?""#.into(),
    };
    tokio::spawn(run(thorin));
    tokio::spawn(run(gandalf));
    loop {
        let msg: String = rx.recv().await.unwrap();
        println!("{msg}");
    }
}
