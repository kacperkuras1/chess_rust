flashMessages = document.querySelectorAll(".flash-message");

function closeFlashMessage(flashMessage) {
    flashMessage.classList.add("fade-out");
    setTimeout(() => {
        flashMessage.remove();
    }, 500);

}


for (let i = 0; i < flashMessages.length; i++) {
    flashMessages[i].children[0].addEventListener("click", () => {
        closeFlashMessage(flashMessages[i]);
    });

    setTimeout(() => {
        closeFlashMessage(flashMessages[i]);
    }, 5000 + 1000 * i);
}
