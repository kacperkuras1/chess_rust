const config = {
  draggable: true,
  position: 'start',
  orientation: color,
  onDrop
}
const board = Chessboard2('board', config);


function onDrop (source) {
  let move = {
    from: source.source,
    to: source.target,
  }
  ws.send(JSON.stringify(move));
}
