export function fmtCents(cents) {
  return new Intl.NumberFormat('es-AR', { style: 'currency', currency: 'ARS' }).format(cents / 100);
}

export function fmtDate(d) {
  return d.toLocaleDateString('es-AR', { day: '2-digit', month: '2-digit', year: 'numeric' })
       + ' ' + d.toLocaleTimeString('es-AR', { hour: '2-digit', minute: '2-digit' });
}

export function fmtQty(item) {
  if (item.sale_mode === 'bulk') {
    const g = item.quantity;
    return g >= 1000 ? `${g}g (${(g / 1000).toFixed(1).replace(/\.0$/, '')}kg)` : `${g}g`;
  }
  return item.quantity === 1 ? '1 unidad' : `${item.quantity} unidades`;
}

export function calcSubtotal(item) {
  return item.sale_mode === 'bulk'
    ? Math.ceil(item.price_cents * item.quantity / 1000)
    : item.price_cents * item.quantity;
}
