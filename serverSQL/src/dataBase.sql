-- Criando usuario
CREATE USER joao WITH PASSWORD '979838870';

-- Criando banco de dados
CREATE DATABASE SauCe  
WITH OWNER = joao  
ENCODING = 'UTF8'  
LC_COLLATE = 'pt_BR.UTF-8'  
LC_CTYPE = 'pt_BR.UTF-8'  
TEMPLATE = template0;  

-- Conecta no banco
\c SauCe

-- Criar tabela
CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    nomeUsuario VARCHAR(50) NOT NULL,
    email VARCHAR(200) UNIQUE NOT NULL,
    senha VARCHAR(100) NOT NULL,
    valor DECIMAL(10,2) NOT NULL DEFAULT 0.00,
    status BOOLEAN NOT NULL,
    criado TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


SELECT * FROM accounts --view rows