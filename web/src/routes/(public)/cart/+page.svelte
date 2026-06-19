<script>
  import { goto } from '$app/navigation';
  import { cart } from '$lib/cart.js';
  import { fmtCents, fmtQty, calcSubtotal } from '$lib/format.js';

  const API = '';
  const WA_NUMBER = '5492236045733';

  let phone = '';
  let email = '';
  let name = '';
  let phoneError = false;
  let emailError = false;
  let submitting = false;

  $: items = Object.values($cart.items);
  $: total = items.reduce((s, i) => s + calcSubtotal(i), 0);
  $: canSubmit = items.length > 0 && !submitting;

  function adjustQty(item, delta) {
    const newQty = item.quantity + delta;
    if (newQty <= 0) cart.removeItem(item.id);
    else cart.setItem(item.id, newQty, [item]);
  }

  async function submitOrder() {
    phoneError = !phone.trim();
    emailError = !email.trim();
    if (phoneError || emailError) return;

    submitting = true;
    try {
      await fetch(`${API}/orders`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          customer_phone: phone.trim(),
          customer_email: email.trim(),
          customer_name:  name.trim() || null,
          items: items.map(i => ({ product_id: i.id, quantity: i.quantity })),
        }),
      });
    } catch (_) { /* silent — WA still opens */ }

    const tableRows = items.flatMap(i => [`• ${i.name} — ${fmtQty(i)} — ${fmtCents(calcSubtotal(i))}`]);
    const msgParts = [
      '*Nuevo pedido — Pichel*',
      '',
      ...tableRows,
      `Total: ${fmtCents(total)}`,
      '',
      'Cliente',
      `Tel: ${phone.trim()}`,
      `Email: ${email.trim()}`,
      ...(name.trim() ? [`Nombre: ${name.trim()}`] : []),
    ];

    window.open(`https://wa.me/${WA_NUMBER}?text=${encodeURIComponent(msgParts.join('\n'))}`, '_blank');

    cart.clear();
    phone = '';
    email = '';
    name = '';
    submitting = false;
    goto('/');
  }
</script>

<svelte:head>
  <title>Tu pedido — Pichel</title>
</svelte:head>

<section style="max-width:680px; margin:0 auto; padding:var(--sp-6) var(--sp-4) var(--sp-16)">
  <div style="display:flex; align-items:center; gap:var(--sp-4); margin-bottom:var(--sp-6)">
    <a href="/" style="color:var(--clr-text-muted); text-decoration:none; font-size:var(--text-sm)">← Volver</a>
    <h2 style="font-family:var(--font-display); font-size:var(--text-xl); font-weight:var(--fw-semibold)">Tu pedido</h2>
  </div>

  {#if $cart.reconcileAlert}
    <div class="cart-alert" style="display:block; margin-bottom:var(--sp-4)">
      {#if $cart.reconcileAlert.repriced.length}
        Precios actualizados:<br>
        {#each $cart.reconcileAlert.repriced as r}
          · {r.name}: {fmtCents(r.from)} → {fmtCents(r.to)}<br>
        {/each}
      {/if}
      {#if $cart.reconcileAlert.removed.length}
        Productos no disponibles (eliminados del pedido):<br>
        {#each $cart.reconcileAlert.removed as n}· {n}<br>{/each}
      {/if}
    </div>
  {/if}

  {#if !items.length}
    <p class="cart-empty">Tu carrito está vacío 🛒</p>
  {:else}
    <div class="cart-items" style="margin-bottom:var(--sp-4)">
      {#each items as item (item.id)}
        <div class="cart-item">
          <div class="ci-info">
            <span class="ci-name">{item.name}</span>
            <span class="ci-qty-label">{fmtQty(item)}</span>
          </div>
          <div class="ci-right">
            <div class="ci-stepper">
              <button class="qty-btn" on:click={() => adjustQty(item, -1)}>−</button>
              <span class="ci-qty-val">{item.quantity}{item.sale_mode === 'bulk' ? 'g' : ''}</span>
              <button class="qty-btn" on:click={() => adjustQty(item, 1)}>+</button>
            </div>
            <span class="ci-sub">{fmtCents(calcSubtotal(item))}</span>
            <button class="ci-remove" on:click={() => cart.removeItem(item.id)} aria-label="Eliminar">✕</button>
          </div>
        </div>
      {/each}
    </div>

    <div class="cart-footer" style="border-top:var(--border); padding-top:var(--sp-4)">
      <div class="cart-total-row" style="margin-bottom:var(--sp-4)">
        <span>Total</span>
        <strong>{fmtCents(total)}</strong>
      </div>

      <label class="cart-persist-row">
        <input type="checkbox" checked={$cart.persist}
          on:change={e => cart.setPersist(e.target.checked)} />
        <span>
          <strong>Guardar pedido para futuras compras</strong>
          <small>El pedido no se borrará al confirmar ni expirará automáticamente</small>
        </span>
      </label>

      <div class="cart-form" style="margin-top:var(--sp-4)">
        <input type="tel" bind:value={phone} placeholder="📱 Teléfono (requerido)"
          autocomplete="tel" class:input-error={phoneError}
          on:input={() => (phoneError = false)} />
        <input type="email" bind:value={email} placeholder="Email (requerido)"
          autocomplete="email" class:input-error={emailError}
          on:input={() => (emailError = false)} />
        <input type="text" bind:value={name} placeholder="Nombre (opcional)" autocomplete="name" />
        <button class="btn-wpp-submit" disabled={!canSubmit} on:click={submitOrder}>
          {#if submitting}
            <span class="spinner"></span> Enviando…
          {:else}
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
              <path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347z"/>
              <path d="M12 0C5.373 0 0 5.373 0 12c0 2.124.558 4.118 1.532 5.845L.057 23.885a.5.5 0 0 0 .61.61l6.04-1.475A11.954 11.954 0 0 0 12 24c6.627 0 12-5.373 12-12S18.627 0 12 0zm0 21.818a9.818 9.818 0 0 1-5.032-1.382l-.36-.214-3.733.911.927-3.63-.234-.375A9.785 9.785 0 0 1 2.182 12C2.182 6.576 6.576 2.182 12 2.182S21.818 6.576 21.818 12 17.424 21.818 12 21.818z"/>
            </svg>
            Enviar pedido por WhatsApp
          {/if}
        </button>
      </div>
    </div>
  {/if}
</section>
