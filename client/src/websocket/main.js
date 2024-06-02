export default function connectSocket() {
  //e.g "http://"
  let ws_path = new URL(window.location.origin + "/api/ws");

  if (ws_path.protocol === "http") {
    ws_path = ws_path.href.replace("http", "ws");
  } else {
    ws_path = ws_path.href.replace("https", "wss");
  }

  const socket = new WebSocket(ws_path);

  addListeners(socket);

  return socket;
}

function addListeners(socket) {
  socket.addEventListener("message", (m) => {
    const msg = m.data.trim();

    switch (msg) {
      case "refresh":
        window.location.reload();
        break;

      default:
        break;
    }

    console.log(msg);
  });
}
