<input type="text" bind:value={nomeusuario} placeholder="Nome de usuário" />
<input type="email" bind:value={email} placeholder="E-mail" />
<input type="password" bind:value={senha} placeholder="Senha" />
<input type="number" bind:value={valor} placeholder="Valor" />
<input type="checkbox" bind:checked={status} /> Ativo
<button on:click={enviar}>Enviar</button>

<p>Usuário: {nomeusuario}</p>
<p>Email: {email}</p>
<p>Senha: {senha}</p>
<p>Valor: {valor}</p>
<p>Status: {status ? "Ativo" : "Inativo"}</p>

<script>
  let nomeusuario = "";
  let email = "";
  let senha = "";
  let valor = 0;
  let status = false;
  let socket;

  function conectarWebSocket() {
    socket = new WebSocket("ws://localhost:8080");

    socket.onopen = () => console.log("Conectado ao WebSocket");
    socket.onclose = () => {
      console.log("WebSocket desconectado. Tentando reconectar...");
      setTimeout(conectarWebSocket, 1000);
    };
  }

  function enviar() {
    if (socket.readyState === WebSocket.OPEN) {
      const dados = {
        nomeusuario,
        email,
        senha,
        valor,
        status
      };
      socket.send(JSON.stringify(dados));
    } else {
      console.log("WebSocket não está pronto, tentando reconectar...");
      conectarWebSocket();
    }
  }

  conectarWebSocket();
</script>
