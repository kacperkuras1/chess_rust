
const email = document.getElementById("email");
const username = document.getElementById("username");
const password = document.getElementById("password");
const confirmPassword = document.getElementById("confirm_password");
const level = document.getElementById("level");

const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
const usernameRegex = /^[a-zA-Z0-9._]+$/;
const passwordRegex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^A-Za-z\d]).{8,}$/;

function validateInput(input, isValid) {
  if (isValid) {
    input.classList.remove("invalid");
    input.classList.add("valid");
  } else {
    input.classList.remove("valid");
    input.classList.add("invalid");
  }
}

email.addEventListener("input", () => {
  validateInput(email, emailRegex.test(email.value));
});

username.addEventListener("input", () => {
  validateInput(username, usernameRegex.test(username.value));
});

password.addEventListener("input", () => {
  validateInput(password, passwordRegex.test(password.value));
  validateInput(confirmPassword, password.value === confirmPassword.value && confirmPassword.value.length > 0);
});

confirmPassword.addEventListener("input", () => {
  validateInput(confirmPassword, password.value === confirmPassword.value && password.value.length > 0);
});

level.addEventListener("change", () => {
  validateInput(level, !!level.value);
});

const form = document.querySelector("form");
form.addEventListener("submit", function (e) {
  const allValid =
    emailRegex.test(email.value) &&
    usernameRegex.test(username.value) &&
    passwordRegex.test(password.value) &&
    password.value === confirmPassword.value &&
    !!level.value;

  if (!allValid) {
    e.preventDefault();
  }
});
