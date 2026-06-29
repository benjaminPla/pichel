<script>
  import { onMount } from 'svelte';
  import { apiFetch } from '$lib/api.js';
  import { fmtCents, fmtDate, fmtQty } from '$lib/format.js';
  import { topbarTitle } from '$lib/adminStore.js';

  topbarTitle.set('Pedidos');

  const PER_PAGE = 25;

  let orders = [];
  let total = 0;
  let currentPage = 1;
  let totalPages = 1;
  let loading = true;

  onMount(() => loadOrders(1));

  async function loadOrders(page) {
    currentPage = page;
    loading = true;
    const res = await apiFetch(`/orders?page=${page}&per_page=${PER_PAGE}`);
    if (!res || !res.ok) { loading = false; return; }
    const data = await res.json();
    orders = data.orders;
    total = data.total;
    totalPages = Math.ceil(total / PER_PAGE);
    loading = false;
  }

  const STATUS_MAP = {
    pending:   { cls: 'badge-pending',   label: 'Pendiente'  },
    closed:    { cls: 'badge-closed',    label: 'Cerrado'    },
    cancelled: { cls: 'badge-cancelled', label: 'Cancelado'  },
  };
</script>

<svelte:head>
  <title>Pedidos — Pichel Admin</title>
</svelte:head>

<div class="section-header section-header--flush">
  <h2>Todos los pedidos</h2>
  {#if total}<span class="text-muted">{total} pedido{total !== 1 ? 's' : ''}</span>{/if}
</div>

{#if loading}
  <div class="table-wrap"><p class="table-empty">Cargando…</p></div>
{:else if !orders.length}
  <div class="table-wrap"><p class="table-empty">Sin pedidos todavía.</p></div>
{:else}
  <!-- Desktop table -->
  <div class="table-wrap orders-table">
    <table>
      <thead><tr>
        <th>Fecha</th>
        <th>Cliente</th>
        <th>Productos</th>
        <th>Total</th>
        <th>Estado</th>
      </tr></thead>
      <tbody>
        {#each orders as o (o.id)}
          {@const s = STATUS_MAP[o.status] ?? { cls: '', label: o.status }}
          <tr>
            <td class="td-meta">{fmtDate(new Date(o.created_at))}</td>
            <td>
              <span class="fw-semibold">{o.customer_phone}</span>
              {#if o.customer_name}<br><small class="text-muted">{o.customer_name}</small>{/if}
            </td>
            <td class="td-items">
              {#each o.items as i}{i.product_name} · {fmtQty(i)}<br>{/each}
            </td>
            <td class="td-amount">{fmtCents(o.total_price_cents)}</td>
            <td><span class="badge {s.cls}">{s.label}</span></td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <!-- Mobile cards -->
  <div class="orders-cards">
    {#each orders as o (o.id)}
      {@const s = STATUS_MAP[o.status] ?? { cls: '', label: o.status }}
      <dl class="order-card">
        <dt>Fecha</dt>
        <dd class="order-card-meta">{fmtDate(new Date(o.created_at))}</dd>

        <dt>Cliente</dt>
        <dd>
          <span class="fw-semibold">{o.customer_phone}</span>
          {#if o.customer_name}<small>{o.customer_name}</small>{/if}
        </dd>

        <dt>Productos</dt>
        <dd class="order-card-items">
          {#each o.items as i}{i.product_name} · {fmtQty(i)}<br>{/each}
        </dd>

        <dt>Total</dt>
        <dd class="fw-semibold">{fmtCents(o.total_price_cents)}</dd>

        <dt>Estado</dt>
        <dd><span class="badge {s.cls}">{s.label}</span></dd>
      </dl>
    {/each}
  </div>

  {#if totalPages > 1}
    <div class="pagination">
      <button class="pagination-btn" disabled={currentPage <= 1}
        on:click={() => loadOrders(currentPage - 1)}>← Anterior</button>
      <span class="pagination-info">Página {currentPage} de {totalPages}</span>
      <button class="pagination-btn" disabled={currentPage >= totalPages}
        on:click={() => loadOrders(currentPage + 1)}>Siguiente →</button>
    </div>
  {/if}
{/if}
