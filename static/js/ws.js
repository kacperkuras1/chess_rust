// let btn = document.getElementById('btnSend');
// btn.addEventListener('click', sendMessage);

const ws = new WebSocket(`ws://localhost:8080/ws/${color}`);

ws.onopen = () => {
  console.log("Połączono z serwerem WebSocket.");
};

ws.onmessage = (event) => {
//   log("Odpowiedź z serwera: " + event.data);
//   console.log(event.data);
  let temp = JSON.parse(event.data);
  let move = temp.from + "-" + temp.to;
  board.move(move);
  console.log(temp);
};

ws.onerror = (error) => {
    console.log("Błąd WebSocket: " + error);
};

ws.onclose = () => {
    console.log("Połączenie zamknięte.");
};

// function sendMessage() {
//   console.log("Wysyłanie wiadomości...");
//   const input = document.getElementById("msg");
//   const text = input.value;
//   const move = {
    // from: "e2",
    // to: "e4"
//   };
//   ws.send(JSON.stringify(move));
//   console.log("Wysyłam JSON:", JSON.stringify(move));
//   log("Wysłano: " + text);
//   input.value = "";
// }

// function log(msg) {
//   const pre = document.getElementById("log");
//   pre.textContent += msg + "\n";
// }
// console.log("Chessboard initialized.");