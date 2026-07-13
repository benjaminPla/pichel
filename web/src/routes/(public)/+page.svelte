<script>
  import { onMount } from 'svelte';
  import { cart } from '$lib/cart.js';
  import { fmtCents, fmtDate } from '$lib/format.js';
  import { SYMBOL_DEFS } from '$lib/symbols.js';

  const API = '';
  const PER_PAGE = 50;

  let products = [];
  let categories = [];
  let loading = true;
  let error = false;
  let currentPage = 1;
  let totalPages = 1;
  let totalCount = 0;
  let priceListUpdatedAt = null;
  let nameFilter = '';
  let categoryFilter = '';
  let nameDebounce;

  onMount(async () => {
    try {
      const catRes = await fetch(`${API}/categories`);
      if (catRes.ok) categories = (await catRes.json()).categories;
    } catch { /* filter bar just won't offer categories */ }
    loadProducts(1);
  });

  async function loadProducts(page) {
    currentPage = page;
    loading = true;
    error = false;
    try {
      const params = new URLSearchParams({ page, per_page: PER_PAGE });
      if (nameFilter.trim()) params.set('name', nameFilter.trim());
      if (categoryFilter) params.set('category_id', categoryFilter);
      const res = await fetch(`${API}/products?${params}`);
      if (!res.ok) throw new Error();
      const { products: data, total, price_list_updated_at } = await res.json();
      products = data;
      totalCount = total;
      totalPages = Math.ceil(total / PER_PAGE);
      priceListUpdatedAt = price_list_updated_at ? new Date(price_list_updated_at) : null;
      if (page === 1 && !nameFilter && !categoryFilter) cart.reconcile(data, data.length === total);
    } catch {
      error = true;
    } finally {
      loading = false;
    }
  }

  function onNameFilterInput() {
    clearTimeout(nameDebounce);
    nameDebounce = setTimeout(() => loadProducts(1), 300);
  }

  let pending = {};

  function stage(p) {
    pending[p.id] = p.sale_mode === 'bulk' ? 50 : 1;
    pending = pending;
  }

  function confirm(p) {
    const qty = pending[p.id];
    if (qty > 0) cart.setItem(p.id, qty, products);
    delete pending[p.id];
    pending = pending;
  }

</script>

<svelte:head>
  <title>Pichel — Almacén Natural</title>
</svelte:head>

<section class="hero">
  <div class="hero-content">
    <h1>Pichel <span>almacén natural.</span></h1>
    <p>Frutos secos, legumbres, harinas, productos sin TACC y veganos</p>
  </div>
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

  <div class="filters-bar">
    <input type="text" placeholder="Buscar por nombre…" bind:value={nameFilter} on:input={onNameFilterInput} />
    <select class="form-select" bind:value={categoryFilter} on:change={() => loadProducts(1)}>
      <option value="">Todas las categorías</option>
      {#each categories as c}
        <option value={c.id}>{c.name}</option>
      {/each}
    </select>
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
      <p class="pl-empty">
        {nameFilter || categoryFilter ? 'Sin productos que coincidan con la búsqueda.' : 'Sin productos por ahora.'}
      </p>
    {:else}
      <div class="products-grid">
        {#each products as p (p.id)}
          {@const item = $cart.items[p.id]}
          {@const qty = item ? item.quantity : 0}
          <div class="product-card" class:pc-inactive={!p.active}>
            {#if p.image_url}
              <img class="pc-img" src={p.image_url} alt={p.name} loading="lazy" />
            {:else}
              <div class="pc-img-empty">🌿</div>
            {/if}
            <div class="pc-body">
              <div class="pc-name">{p.name}</div>
              {#if p.description}<div class="pc-desc">{p.description}</div>{/if}
              {#if p.categories?.length}
                <div class="pc-categories">
                  {#each p.categories as c}<span class="badge badge-pkg">{c.name}</span>{/each}
                </div>
              {/if}
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
                {#if p.active}
                  {#if p.sale_mode === 'bulk'}
                    <span class="pl-mode pl-mode-bulk">A granel</span>
                    <span class="pc-price">{fmtCents(p.price_cents)}<span class="pl-price-unit">/kg</span></span>
                  {:else}
                    <span class="pl-mode pl-mode-pkg">Unidad</span>
                    <span class="pc-price">{fmtCents(p.price_cents)}</span>
                  {/if}
                {:else}
                  <span class="pl-mode pc-unavailable-tag">No disponible</span>
                {/if}
              </div>
              {#if p.active}
                <div class="pc-actions">
                  {#if qty > 0}
                    <input type="number"
                      min={p.sale_mode === 'bulk' ? 50 : 1}
                      step={p.sale_mode === 'bulk' ? 50 : 1}
                      value={qty}
                      on:change={e => {
                        const v = +e.target.value;
                        if (v > 0) cart.setItem(p.id, v, products);
                        else cart.removeItem(p.id);
                      }} />
                    <button class="btn-cart-remove" on:click={() => cart.removeItem(p.id)}>Quitar</button>
                  {:else if pending[p.id] != null}
                    <input type="number"
                      min={p.sale_mode === 'bulk' ? 50 : 1}
                      step={p.sale_mode === 'bulk' ? 50 : 1}
                      bind:value={pending[p.id]} />
                    <button class="btn-cart-add" on:click={() => confirm(p)}>Agregar al carrito</button>
                  {:else}
                    <button class="btn-cart-add" on:click={() => stage(p)}>Seleccionar</button>
                  {/if}
                </div>
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
