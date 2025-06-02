
const game = new Chess()

const config = {
  draggable: true,
  position: game.fen(),
  onDragStart,
  onDrop
}
const board = Chessboard2('board', config);


function onDrop (dropEvt) {
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
    typ: 'move',
    from: dropEvt.source,
    to: dropEvt.target,
  }
  ws.send(JSON.stringify(move));    

  } else {
    return 'snapback'
  }
}


const statusEl = byId('gameStatus')
const fenEl = byId('gameFEN')
const pgnEl = byId('gamePGN')

updateStatus()

function onDragStart (dragStartEvt) {
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

function isWhitePiece (piece) { return /^w/.test(piece) }
function isBlackPiece (piece) { return /^b/.test(piece) }


function updateStatus () {
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

  statusEl.innerHTML = statusHTML
  fenEl.innerHTML = game.fen()
  pgnEl.innerHTML = game.pgn()
}

function byId (id) {
  return document.getElementById(id)
}