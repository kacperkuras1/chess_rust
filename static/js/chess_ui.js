const play_again_btn = document.getElementById("overlay-replay-button");
const home_btn = document.getElementById("overlay-home-button");

const chat_form = document.getElementById('chatForm');

window.addEventListener("beforeunload", (event) => {
  if (gameInProgress) {
    event.preventDefault();
    event.returnValue = "Czy na pewno chcesz opuścić grę? Gra zostanie uznana za przegraną.";
  }
});

function showOverlay(type) {
    const overlay = document.getElementById("game-overlay");
    const title = document.getElementById("overlay-title");
    const message = document.getElementById("overlay-message");
    const buttons = document.querySelectorAll(".overlay-button");
    console.log("Buttons: ", buttons);

    overlay.style.display = "flex";
    buttons.forEach(button => button.style.display = "none");

    switch (type) {
        case "waiting":
            title.innerText = "Czekanie na przeciwnika...";
            message.innerText = "Połączono. Czekam na drugiego gracza.";
            break;
        case "win":
            title.innerText = "🎉 Wygrałeś!";
            message.innerText = "Gratulacje! Pokonałeś przeciwnika.";
            buttons[0].style.display = "inline-block";
            buttons[1].style.display = "inline-block";
            break;
        case "lose":
            title.innerText = "😞 Przegrałeś";
            message.innerText = "Następnym razem pójdzie lepiej.";
            buttons[0].style.display = "inline-block";
            buttons[1].style.display = "inline-block";
            break;
        case "draw":
            title.innerText = "🤝 Remis!";
            message.innerText = "Gra zakończyła się remisem.";
            buttons[0].style.display = "inline-block";
            buttons[1].style.display = "inline-block";
            break;
        default:
            overlay.style.display = "none";
            break;
    }
}



play_again_btn.addEventListener("click", () => {
    location.reload();
});

home_btn.addEventListener("click", () => {
    window.location.href = "/";
});


function hideOverlay() {
    document.getElementById("game-overlay").style.display = "none";
}



chat_form.addEventListener('submit', (e) => {
    e.preventDefault();
    const input = document.getElementById('chatInput');
    const msg = input.value.trim();
    if (!msg) return;

    const chatMessages = document.getElementById('chatMessages');
    const newMsg = document.createElement('div');
    newMsg.className = 'chat-message sent';
    newMsg.textContent = msg;
    chatMessages.appendChild(newMsg);

    chatMessages.scrollTop = chatMessages.scrollHeight;

    input.value = '';

    ws.send(JSON.stringify({ msg_type: 'chat', message: msg }));
});

function get_message(message) {
    const chatMessages = document.getElementById('chatMessages');
    const newMsg = document.createElement('div');
    newMsg.className = 'chat-message received';
    newMsg.textContent = message;
    chatMessages.appendChild(newMsg);

    chatMessages.scrollTop = chatMessages.scrollHeight;

}
