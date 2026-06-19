import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

const CART_KEY = 'pichel_cart';
const CART_TTL = 15 * 24 * 60 * 60 * 1000;

function cartLoad() {
  if (!browser) return { items: {}, persist: false };
  try {
    const raw = localStorage.getItem(CART_KEY);
    if (!raw) return { items: {}, persist: false };
    const data = JSON.parse(raw);
    const persist = !!data.persist;
    if (!persist && Date.now() - data.savedAt > CART_TTL) {
      localStorage.removeItem(CART_KEY);
      return { items: {}, persist: false };
    }
    return { items: data.items || {}, persist };
  } catch { return { items: {}, persist: false }; }
}

function cartSave(state) {
  if (!browser) return;
  localStorage.setItem(CART_KEY, JSON.stringify({
    savedAt: Date.now(),
    items: state.items,
    persist: state.persist,
  }));
}

function createCartStore() {
  const { subscribe, update } = writable({ ...cartLoad(), reconcileAlert: null });

  return {
    subscribe,

    setItem(id, qty, products) {
      update(state => {
        const items = { ...state.items };
        if (qty <= 0) {
          delete items[id];
        } else {
          const p = products.find(p => p.id === id);
          if (!p) return state;
          items[id] = { id, name: p.name, price_cents: p.price_cents, sale_mode: p.sale_mode, quantity: qty };
        }
        const next = { ...state, items };
        cartSave(next);
        return next;
      });
    },

    removeItem(id) {
      update(state => {
        const items = { ...state.items };
        delete items[id];
        const next = { ...state, items };
        cartSave(next);
        return next;
      });
    },

    setPersist(val) {
      update(state => {
        const next = { ...state, persist: val };
        cartSave(next);
        return next;
      });
    },

    clear() {
      update(state => {
        if (state.persist) return state;
        const next = { ...state, items: {} };
        cartSave(next);
        return next;
      });
    },

    reconcile(allProducts, allLoaded) {
      update(state => {
        if (!Object.keys(state.items).length) return { ...state, reconcileAlert: null };
        const items = { ...state.items };
        const removed = [];
        const repriced = [];

        Object.keys(items).forEach(id => {
          const item = items[id];
          const current = allProducts.find(p => p.id === id);
          if (!current) {
            if (allLoaded) { removed.push(item.name); delete items[id]; }
          } else if (current.price_cents !== item.price_cents) {
            repriced.push({ name: item.name, from: item.price_cents, to: current.price_cents });
            items[id] = { ...item, price_cents: current.price_cents, sale_mode: current.sale_mode };
          }
        });

        const hasChanges = removed.length || repriced.length;
        const next = { ...state, items };
        if (hasChanges) cartSave(next);
        return { ...next, reconcileAlert: hasChanges ? { removed, repriced } : null };
      });
    },
  };
}

export const cart = createCartStore();
export const cartCount = derived(cart, $c => Object.keys($c.items).length);
