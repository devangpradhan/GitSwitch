import { getCurrentWindow } from '@tauri-apps/api/window';

const isTauri = () => typeof window !== 'undefined' && Boolean(window.__TAURI_INTERNALS__);

export function Quit() {
  if (isTauri()) {
    getCurrentWindow().close();
  } else if (window.runtime && window.runtime.Quit) {
    window.runtime.Quit();
  } else {
    window.close();
  }
}

export function WindowMinimise() {
  if (isTauri()) {
    getCurrentWindow().minimize();
  } else if (window.runtime && window.runtime.WindowMinimise) {
    window.runtime.WindowMinimise();
  }
}

export function WindowToggleMaximise() {
  if (isTauri()) {
    getCurrentWindow().toggleMaximize();
  } else if (window.runtime && window.runtime.WindowToggleMaximise) {
    window.runtime.WindowToggleMaximise();
  }
}

export function WindowReload() {
  if (window.runtime && window.runtime.WindowReload) {
    window.runtime.WindowReload();
  } else {
    window.location.reload();
  }
}
