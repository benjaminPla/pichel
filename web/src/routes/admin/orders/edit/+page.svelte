<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { apiFetch } from '$lib/api.js';
  import { fmtCents, calcSubtotal } from '$lib/format.js';
  import { toast } from '$lib/toast.js';
  import { topbarTitle } from '$lib/adminStore.js';

  topbarTitle.set('Editar pedido');

  const STATUS_MAP = {
    pending:   { label: 'Pendiente'  },
    closed:    { label: 'Cerrado'    },
    cancelled: { label: 'Cancelado'  },
  };

  // Valid transitions from current status
  const TRANSITIONS = {
    pending:   ['pending', 'closed', 'cancelled'],
    closed:    ['closed'],
    cancelled: ['cancelled'],
  };

  let orderId     = null;
  let loading     = true;
  let submitting  = false;

  // Editable fields
  let customerPhone = '';
  let customerName  = '';
  let customerEmail = '';
  let status        = '';
  let originalStatus = '';

  // Items
  let items = [];            // [{ product_id, product_name, quantity, price_cents_at_time, sale_mode }]
  let allProducts = [];      // for the add-product selector
  let addProductId  = '';
  let addQuantity   = '';

  function itemSubtotal(i) {
    return calcSubtotal({ sale_mode: i.sale_mode, price_cents: i.price_cents_at_time, quantity: i.quantity });
  }
  $: computedTotal = items.reduce((sum, i) => sum + itemSubtotal(i), 0);
  $: availableStatuses = TRANSITIONS[originalStatus] ?? [originalStatus];
  $: isTerminal = originalStatus === 'closed' || originalStatus === 'cancelled';

  onMount(async () => {
    orderId = $page.url.searchParams.get('id');
    if (!orderId) { goto('/admin'); return; }

    const [orderRes, productsRes] = await Promise.all([
      apiFetch(`/orders/${orderId}`),
      apiFetch('/products?page=1&per_page=200'),
    ]);

    if (!orderRes?.ok) { goto('/admin'); return; }

    const o      = await orderRes.json();
    const pData  = productsRes?.ok ? await productsRes.json() : { products: [] };

    customerPhone  = o.customer_phone;
    customerName   = o.customer_name  ?? '';
    customerEmail  = o.customer_email ?? '';
    status         = o.status;
    originalStatus = o.status;
    items          = o.items.map(i => ({ ...i }));
    allProducts    = pData.products ?? [];
    loading        = false;
  });

  function removeItem(idx) {
    items = items.filter((_, i) => i !== idx);
  }

  function addItem() {
    const product = allProducts.find(p => p.id === addProductId);
    if (!product) return;
    const qty = parseInt(addQuantity, 10);
    if (!qty || qty < 1) return;

    const existing = items.findIndex(i => i.product_id === product.id);
    if (existing >= 0) {
      items[existing].quantity = qty;
      items = [...items];
    } else {
      items = [...items, {
        product_id:          product.id,
        product_name:        product.name,
        quantity:            qty,
        price_cents_at_time: product.price_cents,
        sale_mode:           product.sale_mode,
      }];
    }
    addProductId = '';
    addQuantity  = '';
  }

  async function submit(e) {
    e.preventDefault();
    if (!items.length) { toast('El pedido debe tener al menos un producto', 'error'); return; }

    submitting = true;
    try {
      const body = {
        customer_phone: customerPhone || undefined,
        customer_name:  customerName  || null,
        customer_email: customerEmail || null,
        status:         status !== originalStatus ? status : undefined,
        items: items.map(i => ({ product_id: i.product_id, quantity: i.quantity })),
      };

      const res = await apiFetch(`/orders/${orderId}`, {
        method: 'PATCH',
        body: JSON.stringify(body),
      });

      if (!res?.ok) {
        const err = await res?.json().catch(() => ({}));
        toast(err?.error ?? 'Error al guardar', 'error');
        return;
      }

      toast('Pedido actualizado');
      goto('/admin');
    } catch {
      toast('Error del servidor', 'error');
    } finally {
      submitting = false;
    }
  }
</script>

<svelte:head>
  <title>Editar pedido — Pichel Admin</title>
</svelte:head>

{#if loading}
  <div class="form-card"><p class="text-muted">Cargando…</p></div>
{:else}
  <form class="form-stack" on:submit={submit}>

    <!-- Customer -->
    <div class="form-card">
      <h3 class="form-title">Datos del cliente</h3>
      <div class="form-stack">
        <div>
          <label for="o-phone">Teléfono *</label>
          <input id="o-phone" type="text" required bind:value={customerPhone} />
        </div>
        <div>
          <label for="o-name">Nombre</label>
          <input id="o-name" type="text" bind:value={customerName} placeholder="Opcional" />
        </div>
        <div>
          <label for="o-email">Email</label>
          <input id="o-email" type="email" bind:value={customerEmail} placeholder="Opcional" />
        </div>
      </div>
    </div>

    <!-- Status -->
    <div class="form-card">
      <h3 class="form-title">Estado</h3>
      {#if isTerminal}
        <p class="text-muted">Este pedido está en estado terminal y no puede cambiar de estado.</p>
      {:else}
        <select class="form-select" bind:value={status}>
          {#each availableStatuses as s}
            <option value={s}>{STATUS_MAP[s]?.label ?? s}</option>
          {/each}
        </select>
      {/if}
    </div>

    <!-- Items -->
    <div class="form-card">
      <h3 class="form-title">Productos</h3>

      {#if items.length}
        <div class="table-wrap" style="margin-bottom: var(--sp-4)">
          <table>
            <thead><tr>
              <th>Producto</th><th>Cantidad</th><th>Precio unitario</th><th>Subtotal</th><th></th>
            </tr></thead>
            <tbody>
              {#each items as item, idx (item.product_id)}
                <tr>
                  <td class="td-strong">{item.product_name}</td>
                  <td>
                    <input
                      type="number"
                      min={item.sale_mode === 'bulk' ? 50 : 1}
                      step={item.sale_mode === 'bulk' ? 50 : 1}
                      style="width: 80px"
                      bind:value={item.quantity}
                      on:input={() => items = [...items]}
                    />
                    {#if item.sale_mode === 'bulk'}<small class="text-muted"> g</small>{/if}
                  </td>
                  <td class="text-muted">{fmtCents(item.price_cents_at_time)}{#if item.sale_mode === 'bulk'}<small>/kg</small>{/if}</td>
                  <td class="fw-semibold">{fmtCents(itemSubtotal(item))}</td>
                  <td>
                    <button type="button" class="btn btn-danger btn-sm"
                      on:click={() => removeItem(idx)}>Quitar</button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {:else}
        <p class="text-muted" style="margin-bottom: var(--sp-4)">Sin productos.</p>
      {/if}

      <!-- Total -->
      <div style="display: flex; justify-content: flex-end; gap: var(--sp-2); align-items: baseline; margin-bottom: var(--sp-5)">
        <span class="text-muted" style="font-size: var(--text-sm)">Total calculado:</span>
        <span style="font-family: var(--font-display); font-size: var(--text-xl); font-weight: var(--fw-bold); color: var(--clr-accent)">{fmtCents(computedTotal)}</span>
      </div>

      <!-- Add product row -->
      <div class="form-row" style="align-items: end">
        <div>
          <label for="o-add-product">Agregar producto</label>
          <select id="o-add-product" class="form-select" bind:value={addProductId}>
            <option value="">— Seleccionar —</option>
            {#each allProducts as p}
              <option value={p.id}>{p.name}</option>
            {/each}
          </select>
        </div>
        <div>
          <label for="o-add-qty">Cantidad</label>
          <input id="o-add-qty" type="number" min="1" step="1" style="width: 100px" bind:value={addQuantity} />
        </div>
        <button type="button" class="btn btn-ghost" on:click={addItem}>Agregar</button>
      </div>
    </div>

    <!-- Actions -->
    <div class="form-actions">
      <button class="btn btn-primary" type="submit" disabled={submitting}>
        {submitting ? '…' : 'Guardar cambios'}
      </button>
      <a class="btn btn-ghost" href="/admin">Cancelar</a>
    </div>

  </form>
{/if}
