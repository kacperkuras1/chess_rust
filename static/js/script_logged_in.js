const userMenu = document.getElementById('navbarUser');

userMenu.addEventListener('click', (e) => {
  e.stopPropagation();
  userMenu.classList.toggle('active');
});

window.addEventListener('click', () => {
  userMenu.classList.remove('active');
});


