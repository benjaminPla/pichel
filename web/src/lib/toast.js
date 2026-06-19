import { writable } from 'svelte/store';

const { subscribe, set } = writable(null);
let timer;

export const toastStore = { subscribe };

export function toast(msg, type = 'success') {
  clearTimeout(timer);
  set({ msg, type });
  timer = setTimeout(() => set(null), 3000);
}
