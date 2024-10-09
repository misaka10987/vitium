const titlebar = `
<div data-tauri-drag-region class="bg-stone-900 h-6 select-none absolute top-0 left-0 right-0 flex justify-end overflow-hidden">
<div data-tauri-drag-region id="title"
  class="absolute top-0 left-0 right-0 h-6 justify-center text-white text-center align-middle truncate">Vitium</div>
<div class="titlebar-button hover:bg-zinc-500 z-50 justify-center max-h-6 h-6" id="titlebar-minimize">
  <img class="max-h-6 h-6 p-1" src="/assets/svg/win-min.svg" alt="minimize">
</div>
<div class="titlebar-button hover:bg-zinc-500 z-50 justify-center max-h-6 h-6" id="titlebar-maximize">
  <img class="max-h-6 h-6 p-1" src="/assets/svg/win-max.svg" alt="maximize">
</div>
<div class="titlebar-button hover:bg-rose-600 z-50 justify-center max-h-6 h-6" id="titlebar-close">
  <img class="max-h-6 h-6 p-1" src="/assets/svg/win-close.svg" alt="close">
</div>
</div>
`

const set_title = (title: string) => $("#title").text(title)

$(() => {
  $("body").prepend(titlebar)
  const { getCurrentWindow } = TAURI_API.window
  const win = getCurrentWindow()
  $("#titlebar-minimize").on("click", () => { win.minimize() })
  $("#titlebar-maximize").on("click", () => { win.toggleMaximize() })
  $("#titlebar-close").on("click", () => { win.close() })
})
