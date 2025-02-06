use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::PgPool;
use serde::Serialize;

#[derive(Debug, FromRow, Serialize)]
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

async fn inserir_dados(pool: &PgPool, dados: Arc<Dados>) -> Result<(), sqlx::Error> {
    let nome = dados.nomeusuario_reg.lock().await.clone();
    let email = dados.email_reg.lock().await.clone();
    let senha = dados.senha_reg.lock().await.clone();
    let valor = *dados.valor_reg.lock().await;
    let status = *dados.status_reg.lock().await;

    sqlx::query("INSERT INTO accounts (nomeusuario, email, senha, valor, status) VALUES ($1, $2, $3, $4, $5)")
        .bind(nome)
        .bind(email)
        .bind(senha)
        .bind(valor)
        .bind(status)
        .execute(pool)
        .await?;
    Ok(())
}

async fn buscar_contas(pool: &PgPool) -> Result<Vec<Info>, sqlx::Error> {
    let contas = sqlx::query_as::<_, Info>("SELECT * FROM accounts ORDER BY criado DESC")
        .fetch_all(pool)
        .await?;
    Ok(contas)
}

#[tokio::main]
async fn main() {
    let conexao_bd = "postgres://joao:979838870@localhost/sauce";
    let pool = PgPoolOptions::new().max_connections(5).connect(conexao_bd).await.unwrap();

    let dados = Arc::new(Dados {
        nomeusuario_reg: Arc::new(Mutex::new(String::new())),
        email_reg: Arc::new(Mutex::new(String::new())),
        senha_reg: Arc::new(Mutex::new(String::new())),
        valor_reg: Arc::new(Mutex::new(0.0)),
        status_reg: Arc::new(Mutex::new(false)),
    });

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let dados_clone = Arc::clone(&dados);
        let pool_clone = pool.clone();
        let pool_clone_busca = pool.clone();

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = ws_stream.split();

            let write_arc = Arc::new(Mutex::new(write));

            // Enviar contas registradas periodicamente
            tokio::spawn({
                let write_arc = Arc::clone(&write_arc);
                async move {
                    loop {
                        if let Ok(contas) = buscar_contas(&pool_clone_busca).await {
                            let json = serde_json::to_string(&contas).unwrap();
                            let mut write_guard = write_arc.lock().await;
                            if write_guard.send(json.into()).await.is_err() {
                                break;
                            }
                        }
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            });

            while let Some(Ok(msg)) = read.next().await {
                let recebido = msg.to_text().unwrap().to_string();
                println!("Recebido do cliente: {}", recebido);

                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&recebido) {
                    if let (Some(nome), Some(email), Some(senha), Some(valor), Some(status)) = (
                        parsed["nomeusuario"].as_str(),
                        parsed["email"].as_str(),
                        parsed["senha"].as_str(),
                        parsed["valor"].as_f64(),
                        parsed["status"].as_bool(),
                    ) {
                        *dados_clone.nomeusuario_reg.lock().await = nome.to_string();
                        *dados_clone.email_reg.lock().await = email.to_string();
                        *dados_clone.senha_reg.lock().await = senha.to_string();
                        *dados_clone.valor_reg.lock().await = valor;
                        *dados_clone.status_reg.lock().await = status;

                        println!("Dados processados: {:?}", parsed);

                        if let Err(e) = inserir_dados(&pool_clone, Arc::clone(&dados_clone)).await {
                            eprintln!("Erro ao inserir no banco: {}", e);
                        } else {
                            println!("Dados inseridos com sucesso!");
                        }

                        let mut write_guard = write_arc.lock().await;
                        write_guard.send(recebido.into()).await.unwrap();
                    }
                }
            }
        });
    }
}
