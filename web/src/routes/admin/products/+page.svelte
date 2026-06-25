<script>
  import { onMount } from 'svelte';
  import { apiFetch } from '$lib/api.js';
  import { fmtCents } from '$lib/format.js';
  import { SYMBOL_DEFS } from '$lib/symbols.js';
  import { toast } from '$lib/toast.js';
  import { topbarTitle } from '$lib/adminStore.js';

  topbarTitle.set('Productos');

  const PER_PAGE = 50;

  let products = [];
  let total = 0;
  let currentPage = 1;
  let totalPages = 1;
  let loading = true;

  onMount(() => loadProducts(1));

  async function loadProducts(page) {
    currentPage = page;
    loading = true;
    const res = await apiFetch(`/products?page=${page}&per_page=${PER_PAGE}`);
    if (!res || !res.ok) { loading = false; return; }
    const data = await res.json();
    products = data.products;
    total = data.total;
    totalPages = Math.ceil(total / PER_PAGE);
    loading = false;
  }

  async function deleteProduct(id) {
    if (!confirm('¿Eliminar este producto?')) return;
    const res = await apiFetch(`/products/${id}`, { method: 'DELETE' });
    if (!res || !res.ok) { toast('Error al eliminar', 'error'); return; }
    toast('Producto eliminado');
    await loadProducts(currentPage);
  }
</script>

<svelte:head>
  <title>Productos — Pichel Admin</title>
</svelte:head>

<div class="section-header section-header--flush">
  <h2>Todos los productos</h2>
  <div style="display:flex; align-items:center; gap:var(--sp-3)">
    {#if total}<span class="text-muted">{total} productos</span>{/if}
    <a href="/admin/products/create" class="btn btn-primary btn-sm">+ Agregar</a>
  </div>
</div>

<div class="table-wrap">
  {#if loading}
    <p class="table-empty">Cargando…</p>
  {:else if !products.length}
    <p class="table-empty">Sin productos todavía.</p>
  {:else}
    <table>
      <thead><tr>
        <th>Nombre</th><th>Modalidad</th><th>Precio</th><th>Símbolos</th><th></th>
      </tr></thead>
      <tbody>
        {#each products as p (p.id)}
          <tr>
            <td class="td-strong">
              {p.name}
              {#if p.description}<br><small class="text-muted">{p.description}</small>{/if}
            </td>
            <td>
              {#if p.sale_mode === 'bulk'}
                <span class="badge badge-bulk">A granel</span>
              {:else}
                <span class="badge badge-pkg">Unidad</span>
              {/if}
            </td>
            <td>
              {fmtCents(p.price_cents)}
              {#if p.sale_mode === 'bulk'}<small class="text-muted"> /kg</small>{/if}
            </td>
            <td>
              {#if p.symbols?.length}
                {#each p.symbols as s}
                  {#if SYMBOL_DEFS[s]}<span class="symbol-icon">{SYMBOL_DEFS[s].icon}</span>{/if}
                {/each}
              {:else}
                <span class="text-muted">—</span>
              {/if}
            </td>
            <td class="nowrap">
              <a class="btn btn-ghost btn-sm" href="/admin/products/edit?id={p.id}">Editar</a>
              <button class="btn btn-danger btn-sm" on:click={() => deleteProduct(p.id)}>Eliminar</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
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
