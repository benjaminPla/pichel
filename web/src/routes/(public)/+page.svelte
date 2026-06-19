<script>
  import { onMount } from 'svelte';
  import { cart } from '$lib/cart.js';
  import { fmtCents, fmtDate, fmtQty, calcSubtotal } from '$lib/format.js';
  import { SYMBOL_DEFS } from '$lib/symbols.js';

  const API = '';
  const PER_PAGE = 50;

  let products = [];
  let loading = true;
  let error = false;
  let currentPage = 1;
  let totalPages = 1;
  let totalCount = 0;
  let priceListUpdatedAt = null;

  onMount(() => loadProducts(1));

  async function loadProducts(page) {
    currentPage = page;
    loading = true;
    error = false;
    try {
      const res = await fetch(`${API}/products?page=${page}&per_page=${PER_PAGE}`);
      if (!res.ok) throw new Error();
      const { products: data, total, price_list_updated_at } = await res.json();
      products = data;
      totalCount = total;
      totalPages = Math.ceil(total / PER_PAGE);
      priceListUpdatedAt = price_list_updated_at ? new Date(price_list_updated_at) : null;
      if (page === 1) cart.reconcile(data, data.length === total);
    } catch {
      error = true;
    } finally {
      loading = false;
    }
  }

  function addToCart(p) {
    cart.setItem(p.id, p.sale_mode === 'bulk' ? 100 : 1, products);
  }

  function adjustQty(p, delta) {
    const item = $cart.items[p.id];
    const newQty = (item ? item.quantity : 0) + delta;
    if (newQty <= 0) cart.removeItem(p.id);
    else cart.setItem(p.id, newQty, products);
  }

  function onQtyInput(p, value) {
    const qty = parseInt(value);
    if (!qty || qty <= 0) return;
    cart.setItem(p.id, qty, products);
  }

  function onQtyBlur(p, value) {
    const qty = parseInt(value) || 0;
    if (qty <= 0) cart.removeItem(p.id);
  }
</script>

<svelte:head>
  <title>Pichel — Almacén Saludable</title>
</svelte:head>

<section class="hero">
  <h1>Dietética <span>natural de barrio.</span></h1>
  <p>Productos a granel · Frutos secos · Semillas · Hierbas</p>
</section>

<section id="precios">
  <div class="section-header">
    <h2>Lista de precios</h2>
    <div class="text-right">
      {#if totalCount}
        <div class="text-muted">{totalCount} productos</div>
      {/if}
      {#if priceListUpdatedAt}
        <div class="list-updated">Actualizada el {fmtDate(priceListUpdatedAt)}</div>
      {/if}
    </div>
  </div>

  <div id="list-wrap">
    {#if loading}
      <div class="products-grid">
        {#each Array(6) as _}
          <div class="product-card skeleton">
            <div class="pc-img"></div>
            <div class="pc-body">
              <div class="card-name"></div>
              <div class="card-price"></div>
            </div>
          </div>
        {/each}
      </div>
    {:else if error}
      <p class="pl-empty">No se pudo cargar la lista.</p>
    {:else if !products.length}
      <p class="pl-empty">Sin productos por ahora.</p>
    {:else}
      <div class="products-grid">
        {#each products as p (p.id)}
          {@const item = $cart.items[p.id]}
          {@const qty = item ? item.quantity : 0}
          <div class="product-card">
            {#if p.image_url}
              <img class="pc-img" src={p.image_url} alt={p.name} loading="lazy" />
            {:else}
              <div class="pc-img-empty">🌿</div>
            {/if}
            <div class="pc-body">
              <div class="pc-name">{p.name}</div>
              {#if p.description}<div class="pc-desc">{p.description}</div>{/if}
              {#if p.symbols?.length}
                <div class="pc-symbols">
                  {#each p.symbols as s}
                    {#if SYMBOL_DEFS[s]}
                      <span class="pl-sym" data-name={SYMBOL_DEFS[s].name}>{SYMBOL_DEFS[s].icon}</span>
                    {/if}
                  {/each}
                </div>
              {/if}
              <div class="pc-meta">
                {#if p.sale_mode === 'bulk'}
                  <span class="pl-mode pl-mode-bulk">A granel</span>
                  <span class="pc-price">{fmtCents(p.price_cents)}<span class="pl-price-unit">/kg</span></span>
                {:else}
                  <span class="pl-mode pl-mode-pkg">Unidad</span>
                  <span class="pc-price">{fmtCents(p.price_cents)}</span>
                {/if}
              </div>
              {#if qty > 0}
                <div class="qty-stepper" style="display:flex">
                  <button class="qty-btn" on:click={() => adjustQty(p, -1)}>−</button>
                  <input class="qty-input" type="number" min="0" step="1"
                    value={qty}
                    on:input={e => onQtyInput(p, e.target.value)}
                    on:blur={e => onQtyBlur(p, e.target.value)} />
                  <button class="qty-btn" on:click={() => adjustQty(p, 1)}>+</button>
                  {#if p.sale_mode === 'bulk'}<span class="qty-unit">g</span>{/if}
                </div>
                <div class="qty-hint">{fmtQty(item)}</div>
                <div class="qty-subtotal">{fmtCents(calcSubtotal(item))}</div>
              {:else}
                <button class="btn-cart-add" on:click={() => addToCart(p)}>Agregar</button>
              {/if}
            </div>
          </div>
        {/each}
      </div>
      {#if totalPages > 1}
        <div class="pagination">
          <button class="pagination-btn" disabled={currentPage <= 1}
            on:click={() => loadProducts(currentPage - 1)}>← Anterior</button>
          <span class="pagination-info">Página {currentPage} de {totalPages}</span>
          <button class="pagination-btn" disabled={currentPage >= totalPages}
            on:click={() => loadProducts(currentPage + 1)}>Siguiente →</button>
        </div>
      {/if}
    {/if}
  </div>
</section>
