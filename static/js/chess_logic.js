
const game = new Chess()
let canDrag = true;

const config = {
  draggable: true,
  position: game.fen(),
  onDragStart,
  onDrop
}

const board = Chessboard2('board', config);


function onDrop(dropEvt) {
  const move = game.move({
    from: dropEvt.source,
    to: dropEvt.target,
    promotion: 'q'
  })

  board.clearCircles()

  if (move) {
    board.fen(game.fen(), () => {
      updateStatus()
    })

    let move = {
      msg_type: 'move',
      from: dropEvt.source,
      to: dropEvt.target,
    }
    ws.send(JSON.stringify(move));
    renderMovesFromPGN(game.pgn());
    console.log(game.pgn());
    canDrag = false;
  } else {
    return 'snapback'
  }
}



updateStatus()

function onDragStart(dragStartEvt) {
  if (!canDrag) return false
  if (game.game_over()) return false

  if (game.turn() === 'w' && !isWhitePiece(dragStartEvt.piece)) return false
  if (game.turn() === 'b' && !isBlackPiece(dragStartEvt.piece)) return false

  const legalMoves = game.moves({
    square: dragStartEvt.square,
    verbose: true
  })

  legalMoves.forEach((move) => {
    board.addCircle(move.to)
  })
}

function isWhitePiece(piece) { return /^w/.test(piece) }
function isBlackPiece(piece) { return /^b/.test(piece) }


function updateStatus() {
  let statusHTML = ''
  const whosTurn = game.turn() === 'w' ? 'White' : 'Black'

  if (!game.game_over()) {
    if (game.in_check()) statusHTML = whosTurn + ' is in check! '
    statusHTML = statusHTML + whosTurn + ' to move.'
  } else if (game.in_checkmate() && game.turn() === 'w') {
    statusHTML = 'Game over: white is in checkmate. Black wins!'
  } else if (game.in_checkmate() && game.turn() === 'b') {
    statusHTML = 'Game over: black is in checkmate. White wins!'
  } else if (game.in_stalemate() && game.turn() === 'w') {
    statusHTML = 'Game is drawn. White is stalemated.'
  } else if (game.in_stalemate() && game.turn() === 'b') {
    statusHTML = 'Game is drawn. Black is stalemated.'
  } else if (game.in_threefold_repetition()) {
    statusHTML = 'Game is drawn by threefold repetition rule.'
  } else if (game.insufficient_material()) {
    statusHTML = 'Game is drawn by insufficient material.'
  } else if (game.in_draw()) {
    statusHTML = 'Game is drawn by fifty-move rule.'
  }

}


function renderMovesFromPGN(pgn) {
  // Najpierw usuń nagłówki (linie zaczynające się od '[')
  const lines = pgn.split('\n').filter(line => !line.startsWith('[')).join(' ');
  
  // Podziel na tokeny po spacji
  const tokens = lines.trim().split(/\s+/);
  
  const moves = [];
  let currentMoveNumber = 0;
  let currentWhiteMove = null;
  let currentBlackMove = null;
  
  // Przechodzimy token po tokenie
  for (let token of tokens) {
    // Ignoruj puste lub wynik partii (np. 1-0, 0-1, 1/2-1/2)
    if (!token || token.match(/^(1-0|0-1|1\/2-1\/2|\*)$/)) {
      continue;
    }
    
    // Jeżeli token jest numerem ruchu (np. "1.", "2."), to aktualizujemy numer ruchu
    if (token.endsWith('.')) {
      currentMoveNumber = parseInt(token);
      currentWhiteMove = null;
      currentBlackMove = null;
    } else {
      // Jeśli nie mamy białego ruchu na ten ruch, to teraz go dodajemy
      if (currentWhiteMove === null) {
        currentWhiteMove = token;
      } else if (currentBlackMove === null) {
        currentBlackMove = token;
      }
      
      // Jeśli mamy już ruch biały i czarny, to zapisujemy i zerujemy dla kolejnego ruchu
      if (currentWhiteMove !== null && currentBlackMove !== null) {
        moves.push({
          number: currentMoveNumber,
          white: currentWhiteMove,
          black: currentBlackMove,
        });
        currentWhiteMove = null;
        currentBlackMove = null;
      }
    }
  }
  
  // Jeśli ostatni ruch ma tylko białego (np. gra się zakończyła po ruchu białych)
  if (currentWhiteMove !== null && currentBlackMove === null) {
    moves.push({
      number: currentMoveNumber,
      white: currentWhiteMove,
      black: null,
    });
  }
  
  // Budujemy HTML
  let html = '';
  for (const move of moves) {
    html += `<div class="move-pair">
      <span class="move-number">${move.number}.</span>
      <span class="move-white">${move.white}</span>
      ${move.black ? `<span class="move-black">${move.black}</span>` : ''}
    </div>`;
  }
  
  // Wstawiamy do kontenera
  document.getElementById('movesList').innerHTML = html;
}
