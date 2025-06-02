// let btn = document.getElementById('btnSend');
// btn.addEventListener('click', sendMessage);

let ws = null;

fetch("/get_jwt")
  .then(res => {
    if (!res.ok) {
      throw new Error("Nie zalogowany");
    }
    return res.json();
  })
  .then(data => {
    const token = data.token;

    // "joinujemy" WebSocket dopiero po otrzymaniu tokena
    ws = new WebSocket(`ws://localhost:8080/ws/${token}`);

    ws.onopen = () => {
      console.log("Połączono z serwerem WebSocket.");
    };

    ws.onmessage = (event) => {
      let temp = JSON.parse(event.data);
      if (temp.typ === "move"){
      let move = temp.from + "-" + temp.to;
      game.move({
        from: temp.from,
        to: temp.to,
        promotion: 'q'
      });
      board.move(move);
      }
      console.log(temp);
    };

    ws.onerror = (error) => {
      console.log("Błąd WebSocket: " + error);
    };

    ws.onclose = () => {
      console.log("Połączenie zamknięte.");
    };
  })
  .catch(err => {
    console.error("Błąd pobierania JWT:", err);
    window.location.href = "/login";
  });



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