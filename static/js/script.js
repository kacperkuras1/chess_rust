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
  }, 3000 + 1000 * i);
}


function fetchWithAuth(url, options = {}) {
  const token = localStorage.getItem('jwt_token');
  options.headers = {
    ...options.headers,
    'Authorization': `Bearer ${token}`,
  };

  return fetch(url, options).then(res => {
    if (res.status === 401) {
      window.location.href = '/login';
      return Promise.reject('Unauthorized');
    }
    return res;
  });
}

fetchWithAuth('/')
  .then(res => res.json())
  .then(data => {
    console.log(data);
  });
