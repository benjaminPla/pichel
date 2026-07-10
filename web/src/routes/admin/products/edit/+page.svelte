<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { apiFetch, apiUpload } from '$lib/api.js';
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
  let active = true;
  let plu = null;
  let name = '';
  let description = '';
  let saleMode = '';
  let priceStr = '';
  let selectedSymbols = [];
  let imageUrl = '';
  let imageFile = null;
  let previewUrl = '';
  let submitting = false;

  $: isBulk = saleMode === 'bulk';

  onMount(async () => {
    productId = $page.url.searchParams.get('id');
    if (!productId) { goto('/admin/products'); return; }

    const res = await apiFetch(`/products/${productId}`);
    if (!res || !res.ok) { goto('/admin/products'); return; }
    const p = await res.json();
    active          = p.active ?? true;
    plu             = p.plu;
    name            = p.name;
    description     = p.description || '';
    saleMode        = p.sale_mode;
    priceStr        = (p.price_cents / 100).toFixed(2);
    selectedSymbols = p.symbols || [];
    imageUrl        = p.image_url || '';
    previewUrl      = p.image_url || '';
  });

  function handleImageChange(e) {
    const file = e.target.files?.[0];
    if (!file) return;
    imageFile = file;
    previewUrl = URL.createObjectURL(file);
  }

  async function submit(e) {
    e.preventDefault();
    const price_cents = Math.round(parseFloat(priceStr) * 100);
    if (!name || !saleMode || isNaN(price_cents) || price_cents < 1) return;

    submitting = true;
    try {
      if (imageFile) {
        const fd = new FormData();
        fd.append('image', imageFile);
        const up = await apiUpload('/products/image', fd);
        if (!up || !up.ok) { toast('Error al subir imagen', 'error'); return; }
        imageUrl = (await up.json()).url;
      }

      const res = await apiFetch(`/products/${productId}`, {
        method: 'PATCH',
        body: JSON.stringify({
          active,
          name:        name.trim(),
          description: description.trim() || null,
          sale_mode:   saleMode,
          price_cents,
          symbols:     selectedSymbols,
          image_url:   imageUrl || null,
        }),
      });
      if (!res || !res.ok) { toast('Error al guardar', 'error'); return; }
      goto('/admin/products');
    } catch { toast('Error del servidor', 'error'); }
    finally   { submitting = false; }
  }
</script>

<svelte:head>
  <title>Editar producto — Pichel Admin</title>
</svelte:head>

<div class="form-card">
  <form class="form-stack" on:submit={submit}>
    <div>
      <label for="p-name">Nombre *</label>
      <input id="p-name" type="text" maxlength="100" required bind:value={name} />
    </div>
    <div>
      <label for="p-plu">PLU</label>
      <input id="p-plu" type="text" value={plu} disabled />
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
    <div class="form-toggle-row">
      <input id="p-active" type="checkbox" bind:checked={active} />
      <label for="p-active" class="form-toggle-label">Producto activo</label>
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
      <label for="p-img">Imagen</label>
      {#if previewUrl}
        <img src={previewUrl} alt="Vista previa" class="img-preview" />
      {/if}
      <input id="p-img" type="file" accept="image/jpeg,image/png,image/webp"
        on:change={handleImageChange} />
    </div>
    <div class="form-actions">
      <button class="btn btn-primary" type="submit" disabled={submitting}>
        {submitting ? '…' : 'Guardar cambios'}
      </button>
      <a class="btn btn-ghost" href="/admin/products">Cancelar</a>
    </div>
  </form>
</div>
