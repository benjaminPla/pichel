<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { apiFetch } from '$lib/api.js';
  import { toast } from '$lib/toast.js';
  import { topbarTitle } from '$lib/adminStore.js';

  topbarTitle.set('Editar producto');

  const SYMBOLS = [
    { value: 'vegan',         label: '🌱 Vegano'                },
    { value: 'vegetarian',    label: '🥕 Vegetariano'           },
    { value: 'gluten_free',   label: '🌾 Sin TACC'              },
    { value: 'lactose_free',  label: '🥛 Sin lactosa'           },
    { value: 'organic',       label: '🌿 Orgánico'              },
    { value: 'sugar_free',    label: '🍃 Sin azúcar agregada'   },
    { value: 'contains_nuts', label: '🥜 Contiene frutos secos' },
    { value: 'high_protein',  label: '💪 Alto en proteína'      },
    { value: 'no_added_salt', label: '🧂 Sin sal agregada'      },
  ];

  let productId = null;
  let name = '';
  let description = '';
  let saleMode = '';
  let priceStr = '';
  let selectedSymbols = [];
  let imageUrl = '';
  let submitting = false;

  $: isBulk = saleMode === 'bulk';

  onMount(async () => {
    productId = $page.url.searchParams.get('id');
    if (!productId) { goto('/admin/products'); return; }

    const res = await apiFetch(`/products/${productId}`);
    if (!res || !res.ok) { goto('/admin/products'); return; }
    const p = await res.json();
    name            = p.name;
    description     = p.description || '';
    saleMode        = p.sale_mode;
    priceStr        = (p.price_cents / 100).toFixed(2);
    selectedSymbols = p.symbols || [];
    imageUrl        = p.image_url || '';
  });

  async function submit(e) {
    e.preventDefault();
    const price_cents = Math.round(parseFloat(priceStr) * 100);
    if (!name || !saleMode || isNaN(price_cents) || price_cents < 1) return;

    submitting = true;
    try {
      const res = await apiFetch(`/products/${productId}`, {
        method: 'PATCH',
        body: JSON.stringify({
          name:        name.trim(),
          description: description.trim() || null,
          sale_mode:   saleMode,
          price_cents,
          symbols:     selectedSymbols,
          image_url:   imageUrl.trim() || null,
        }),
      });
      if (!res || !res.ok) { toast('Error al guardar', 'error'); return; }
      goto('/admin/products');
    } catch { toast('Error del servidor', 'error'); }
    finally   { submitting = false; }
  }
</script>

<div class="form-card">
  <form class="form-stack" on:submit={submit}>
    <div>
      <label for="p-name">Nombre *</label>
      <input id="p-name" type="text" maxlength="100" required bind:value={name} />
    </div>
    <div>
      <label for="p-desc">Descripción</label>
      <textarea id="p-desc" maxlength="255" rows="3" bind:value={description}></textarea>
    </div>
    <div>
      <label for="p-salemode">Modalidad *</label>
      <select id="p-salemode" class="form-select" required bind:value={saleMode}>
        <option value="">— Seleccionar —</option>
        <option value="bulk">A granel</option>
        <option value="unit">Unidad</option>
      </select>
    </div>
    <div>
      <label for="p-price">
        Precio *
        {#if isBulk}<small class="form-label-note">(por kg)</small>{/if}
      </label>
      <input id="p-price" type="number" min="0.01" step="0.01" required bind:value={priceStr} />
    </div>
    <div>
      <label for="p-symbols">Símbolos</label>
      <select id="p-symbols" class="form-select select-multi" multiple bind:value={selectedSymbols}>
        {#each SYMBOLS as s}
          <option value={s.value}>{s.label}</option>
        {/each}
      </select>
    </div>
    <div>
      <label for="p-img">URL de imagen</label>
      <input id="p-img" type="text" placeholder="/images/products/nombre.webp" bind:value={imageUrl} />
    </div>
    <div class="form-actions">
      <button class="btn btn-primary" type="submit" disabled={submitting}>
        {submitting ? '…' : 'Guardar cambios'}
      </button>
      <a class="btn btn-ghost" href="/admin/products">Cancelar</a>
    </div>
  </form>
</div>
