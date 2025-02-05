use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use std::sync::{Arc, Mutex};

#[derive(Debug, FromRow)]
struct Info {
    id: i32,
    nomeusuario: String,
    email: String,
    senha: String,
    valor: f64,
    status: bool,
    criado: NaiveDateTime,
}

#[derive(Debug, Clone)]
struct Dados {
    nomeusuario_reg: Arc<Mutex<String>>,
    email_reg: Arc<Mutex<String>>,
    senha_reg: Arc<Mutex<String>>,
    valor_reg: Arc<Mutex<f64>>,
    status_reg: Arc<Mutex<bool>>,
}

#[tokio::main]
async fn main() {
    let conexao_bd = "postgres://joao:979838870@localhost/sauce";
    let pool = PgPoolOptions::new().max_connections(5).connect(conexao_bd).await.unwrap();
    let sauce: Info = sqlx::query_as::<_, Info>("SELECT id, nomeusuario, email, senha, valor, status, criado FROM accounts LIMIT 1").fetch_one(&pool).await.unwrap();

    let dados = Dados {
        nomeusuario_reg: Arc::new(Mutex::new(String::new())),
        email_reg: Arc::new(Mutex::new(String::new())),
        senha_reg: Arc::new(Mutex::new(String::new())),
        valor_reg: Arc::new(Mutex::new(0.0)),
        status_reg: Arc::new(Mutex::new(false)),
    };

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    
    while let Ok((stream, _)) = listener.accept().await {
        let dados_clone = dados.clone();
        
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = ws_stream.split();
            
            if let Some(Ok(msg)) = read.next().await {
                let recebido = msg.to_text().unwrap().to_string();
                
                *dados_clone.nomeusuario_reg.lock().unwrap() = recebido.clone();
                println!("Recebido e salvo: {}", recebido);
                
                write.send(recebido.into()).await.unwrap();
            }
        });
    }

    loop {
        println!("Nome de usuário atual: {}", *dados.nomeusuario_reg.lock().unwrap());
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
