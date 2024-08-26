use rand::{seq::SliceRandom, Rng};
use std::{collections::HashMap, io, time::Duration};
use tokio::sync::mpsc;

enum Kind {
    Npc,
    LocalPlayer,
}

enum Command {
    Say(String),
    Emit(String),
}

struct Object {
    name: String,
    kind: Kind,
    commands: Vec<Command>,
}

impl Object {
    async fn say(&self, tx: &mpsc::Sender<String>, msg: &str) {
        let msg = format!("{} says \"{}\"", self.name, msg);
        tx.send(msg).await.unwrap();
    }
    
    async fn emit(&self, tx: &mpsc::Sender<String>, msg: &str) {
        let msg = format!("{} {}", self.name, msg);
        tx.send(msg).await.unwrap();
    }

    async fn exec(&self, tx: &mpsc::Sender<String>, cmd: &Command) {
        match cmd {
            Command::Say(msg) => self.say(tx, msg).await,
            Command::Emit(msg) => self.emit(tx, msg).await,
        }
    }
}

async fn run_npc(obj: Object, tx: mpsc::Sender<String>) {
    loop {
        let delay = rand::thread_rng().gen_range(5..10);
        tokio::time::sleep(Duration::from_secs(delay)).await;
        let cmd = obj.commands.choose(&mut rand::thread_rng()).unwrap();
        obj.exec(&tx, cmd).await;
    }
}

async fn run_player(obj: Object, tx: mpsc::Sender<String>) {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        obj.exec(&tx, &Command::Say(line.trim_end().into())).await;
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);
    let objects = HashMap::from([
        (
            "thorin",
            Object {
                name: "Thorin".into(),
                kind: Kind::Npc,
                commands: vec![
                    Command::Say("Hurry up".into()),
                    Command::Emit("sits down and starts singing about gold".into()),
                ],
            },
        ),
        (
            "gandalf",
            Object {
                name: "Gandalf".into(),
                kind: Kind::Npc,
                commands: vec![
                    Command::Say("What's this?".into()),
                    Command::Emit("wanders around picking things up".into()),
                ],
            },
        ),
        (
            "frodo",
            Object {
                name: "Frodo".into(),
                kind: Kind::LocalPlayer,
                commands: Vec::new(),
            },
        ),
    ]);
    for (name, obj) in objects {
        match obj.kind {
            Kind::Npc => {
                println!("Starting NPC {name}");
                tokio::spawn(run_npc(obj, tx.clone()));
            }
            Kind::LocalPlayer => {
                println!("Starting local player {name}");
                tokio::spawn(run_player(obj, tx.clone()));
            }
        }
    }
    loop {
        let msg: String = rx.recv().await.unwrap();
        println!("{msg}");
    }
}
