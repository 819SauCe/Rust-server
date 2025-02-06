<style>
  .container {
    max-width: 400px;
    margin: 40px auto;
    padding: 20px;
    background: #fff;
    border-radius: 10px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
    text-align: center;
  }

  h2 {
    margin-bottom: 15px;
    font-size: 1.5em;
    color: #333;
  }

  .input {
    width: 100%;
    padding: 10px;
    margin-bottom: 10px;
    border: 1px solid #ccc;
    border-radius: 5px;
    font-size: 1em;
  }

  .button {
    width: 100%;
    padding: 10px;
    background: #007bff;
    color: #fff;
    border: none;
    border-radius: 5px;
    font-size: 1em;
    cursor: pointer;
  }

  .button:hover {
    background: #0056b3;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 5px;
    margin: 10px 0;
    font-size: 1em;
  }

  p {
    margin: 5px 0;
    color: #555;
  }

  .account-container {
    position: fixed;
    top: 20px;
    right: 20px;
    width: 300px;
    background: #f9f9f9;
    border-radius: 10px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
    padding: 15px;
    overflow-y: auto;
    max-height: 400px;
  }

  .account-container h3 {
    margin-bottom: 10px;
    font-size: 1.3em;
    color: #333;
    text-align: center;
  }

  .account {
    background: #fff;
    padding: 10px;
    border-radius: 5px;
    margin-bottom: 10px;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
  }

  .account p {
    margin: 5px 0;
    font-size: 0.9em;
    color: #555;
  }

  .account strong {
    color: #000;
  }
</style>

<div class="container">
  <h2>Anotador de Contas</h2>

  <input class="input" type="text" bind:value={nomeusuario} placeholder="Nome de usuário" />
  <input class="input" type="email" bind:value={email} placeholder="E-mail" />
  <input class="input" type="password" bind:value={senha} placeholder="Senha" />
  <input class="input" type="number" bind:value={valor} placeholder="Valor" />

  <label class="checkbox-label">
    <input type="checkbox" bind:checked={status} />
    <span>Ativo</span>
  </label>

  <button class="button" on:click={enviar}>Enviar</button>
  <p id="mensagem" style="display: none; color: green;">Enviado com sucesso!</p>

  {#if nomeusuario}<p>Usuário: {nomeusuario}</p>{/if}
  {#if email}<p>Email: {email}</p>{/if}
  {#if senha}<p>Senha: {senha}</p>{/if}
  {#if valor}<p>Valor: {valor}</p>{/if}
  {#if status !== null}
    <p>Status: {status ? "Ativo" : "Inativo"}</p>
  {/if}
</div>

<div class="account-container">
  <h3>Contas Registradas</h3>
  <div id="account-list"></div>
</div>


<script>
  let nomeusuario = "";
  let email = "";
  let senha = "";
  let valor = null;
  let status = false;
  let socket;

  function conectarWebSocket() {
    socket = new WebSocket("ws://localhost:8080");

    socket.onopen = () => console.log("Conectado ao WebSocket");
    socket.onclose = () => {
      console.log("WebSocket desconectado. Tentando reconectar...");
      setTimeout(conectarWebSocket, 1000);
    };

    socket.onmessage = (event) => {
    const contas = JSON.parse(event.data);
    atualizarContas(contas);
  };
  }

  function atualizarContas(contas) {
    const lista = document.getElementById("account-list");
    lista.innerHTML = "";
    contas.forEach(({ nomeusuario, email, valor, status }) => {
      const div = document.createElement("div");
      div.className = "account";
      div.innerHTML = `
        <p><strong>Usuário:</strong> ${nomeusuario}</p>
        <p><strong>Email:</strong> ${email}</p>
        <p><strong>Valor:</strong> R$ ${valor}</p>
        <p><strong>Status:</strong> ${status ? "Ativo" : "Inativo"}</p>
      `;
      lista.appendChild(div);
    });
  }


  function enviar() {
  console.log("Tentando enviar dados...");
  if (!nomeusuario && !email && !senha && (valor === null || valor === 0)) {
    console.log("Campos vazios. Cancelando envio.");
    return;
  }

  if (socket.readyState === WebSocket.OPEN) {
    const dados = { nomeusuario, email, senha, valor, status };
    console.log("Enviando dados:", dados);
    socket.send(JSON.stringify(dados));

    document.getElementById("mensagem").style.display = "block";
    setTimeout(() => {
      document.getElementById("mensagem").style.display = "none";
    }, 2000);
  } else {
    console.log("WebSocket não está pronto, tentando reconectar...");
    conectarWebSocket();
  }
}

  conectarWebSocket();
</script>
