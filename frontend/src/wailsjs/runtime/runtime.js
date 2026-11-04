// Wails v2 runtime wrapper
export function Quit() {
  if (window.runtime && window.runtime.Quit) {
    window.runtime.Quit();
  } else {
    console.log('[Wails Runtime] Quit requested');
  }
}

export function WindowMinimise() {
  if (window.runtime && window.runtime.WindowMinimise) {
    window.runtime.WindowMinimise();
  } else {
    console.log('[Wails Runtime] WindowMinimise requested');
  }
}

export function WindowToggleMaximise() {
  if (window.runtime && window.runtime.WindowToggleMaximise) {
    window.runtime.WindowToggleMaximise();
  } else {
    console.log('[Wails Runtime] WindowToggleMaximise requested');
  }
}

export function WindowReload() {
  if (window.runtime && window.runtime.WindowReload) {
    window.runtime.WindowReload();
  } else {
    window.location.reload();
  }
}
