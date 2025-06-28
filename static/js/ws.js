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

    ws = new WebSocket(`ws://localhost:8080/ws/${token}`);

    ws.onopen = () => {
      console.log("Połączono z serwerem WebSocket.");
    };

    ws.onmessage = (event) => {
      console.log("Otrzymano wiadomość:", event.data);
      let message_json = JSON.parse(event.data);
      if (message_json.msg_type === "move") {
        let move = message_json.from + "-" + message_json.to;
        game.move({
          from: message_json.from,
          to: message_json.to,
          promotion: 'q'
        });
        canDrag = true;
        renderMovesFromPGN(game.pgn());
        board.move(move);
      }
      else if (message_json.msg_type === "chat") {
        get_message(message_json.message);
        console.log("Otrzymano wiadomość czatu:", message_json.message);
      }
      else if (message_json.msg_type === "game_status") {
        switch (message_json.status) {
          case "waiting":
            showOverlay("waiting");
            break;
          case "playing":
            gameInProgress = true;
            board.orientation(message_json.color);
            game.clear();
            board.position('start');
            game.reset();
            console.log("Gra rozpoczęta, kolor:", message_json.color);
            if (message_json.color === "white") {
              canDrag = true;
            }
            hideOverlay();
            document.getElementById("opponent-name").innerHTML = message_json.opponent_username + " (" + message_json.opponent_elo + ")";
            break;
          case "win":
            gameInProgress = false;
            showOverlay("win");
            break;
          case "lose":
            gameInProgress = false;
            showOverlay("lose");
            break;
          case "draw":
            gameInProgress = false;
            showOverlay("draw");
            break;
        }
      }
      else if (message_json.msg_type === "invalid_move") {
        board.position(message_json.fen);
        canDrag = true;
      }
      console.log(message_json);
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


