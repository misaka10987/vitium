
const titlebar = `
<div data-tauri-drag-region class="bg-stone-800 h-6 select-none fixed top-0 left-0 right-0 flex justify-end">
<div data-tauri-drag-region id="title"
  class="fixed top-0 left-0 right-0 h-6 justify-center text-white text-center align-middle z-40">Vitium</div>
<div class="titlebar-button p-1 hover:bg-zinc-500 z-50 justify-center" id="titlebar-minimize">
  <img src="/assets/svg/win-min.svg" alt="minimize">
</div>
<div class="titlebar-button p-1 hover:bg-zinc-500 z-50 justify-center" id="titlebar-maximize">
  <img src="/assets/svg/win-max.svg" alt="maximize">
</div>
<div class="titlebar-button p-1 hover:bg-rose-600 z-50 justify-center" id="titlebar-close">
  <img src="/assets/svg/win-close.svg" alt="close">
</div>
</div>
`

$(() => {
  $("body").prepend(titlebar)
  const { getCurrentWindow } = TAURI_API.window
  const win = getCurrentWindow()
  $("#titlebar-minimize").on("click", () => { win.minimize() })
  $("#titlebar-maximize").on("click", () => { win.toggleMaximize() })
  $("#titlebar-close").on("click", () => { win.close() })
})
