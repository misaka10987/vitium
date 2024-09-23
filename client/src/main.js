const { invoke } = window.__TAURI__.tauri;

let hide = (id) => document.getElementById(id).hidden = true;

let show = (id) => document.getElementById(id).hidden = false;

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#connect").addEventListener("submit", (e) => {
    e.preventDefault();
    hide("connect");
  });
});
