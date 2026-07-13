<script>
  import { onMount } from 'svelte';
  import { apiFetch } from '$lib/api.js';
  import { toast } from '$lib/toast.js';
  import { topbarTitle } from '$lib/adminStore.js';

  topbarTitle.set('Categorías');

  let categories = [];
  let loading = true;
  let newName = '';
  let submitting = false;
  let editingId = null;
  let editName = '';

  onMount(loadCategories);

  async function loadCategories() {
    loading = true;
    const res = await apiFetch('/categories');
    if (!res || !res.ok) { loading = false; return; }
    const data = await res.json();
    categories = data.categories;
    loading = false;
  }

  async function addCategory(e) {
    e.preventDefault();
    const name = newName.trim();
    if (!name) return;
    submitting = true;
    try {
      const res = await apiFetch('/categories', { method: 'POST', body: JSON.stringify({ name }) });
      if (!res) return;
      if (!res.ok) { toast((await res.json()).error || 'Error al crear la categoría', 'error'); return; }
      newName = '';
      toast('Categoría agregada');
      await loadCategories();
    } finally { submitting = false; }
  }

  function startEdit(c) {
    editingId = c.id;
    editName = c.name;
  }

  function cancelEdit() {
    editingId = null;
    editName = '';
  }

  async function saveEdit(id) {
    const name = editName.trim();
    if (!name) return;
    const res = await apiFetch(`/categories/${id}`, { method: 'PATCH', body: JSON.stringify({ name }) });
    if (!res) return;
    if (!res.ok) { toast((await res.json()).error || 'Error al guardar', 'error'); return; }
    cancelEdit();
    toast('Categoría actualizada');
    await loadCategories();
  }

  async function deleteCategory(id) {
    if (!confirm('¿Eliminar esta categoría? Se quitará de todos los productos que la tengan asignada.')) return;
    const res = await apiFetch(`/categories/${id}`, { method: 'DELETE' });
    if (!res || !res.ok) { toast('Error al eliminar', 'error'); return; }
    toast('Categoría eliminada');
    await loadCategories();
  }
</script>

<svelte:head>
  <title>Categorías — Pichel Admin</title>
</svelte:head>

<div class="section-header section-header--flush">
  <h2>Todas las categorías</h2>
  {#if categories.length}<span class="text-muted">{categories.length} categorías</span>{/if}
</div>

<div class="form-card">
  <form class="form-stack" on:submit={addCategory} style="flex-direction:row; align-items:flex-end; flex-wrap:wrap">
    <div style="flex:1; min-width:200px">
      <label for="c-name">Nombre nueva categoría *</label>
      <input id="c-name" type="text" maxlength="100" required bind:value={newName} />
    </div>
    <div class="form-actions">
      <button class="btn btn-primary" type="submit" disabled={submitting}>
        {submitting ? '…' : '+ Agregar'}
      </button>
    </div>
  </form>
</div>

{#if loading}
  <div class="table-wrap"><p class="table-empty">Cargando…</p></div>
{:else if !categories.length}
  <div class="table-wrap"><p class="table-empty">Sin categorías todavía.</p></div>
{:else}
  <!-- Desktop table -->
  <div class="table-wrap orders-table">
    <table>
      <thead><tr><th>Nombre</th><th class="nowrap">Acciones</th></tr></thead>
      <tbody>
        {#each categories as c (c.id)}
          <tr>
            <td class="td-strong">
              {#if editingId === c.id}
                <input type="text" maxlength="100" bind:value={editName} />
              {:else}
                {c.name}
              {/if}
            </td>
            <td class="nowrap">
              {#if editingId === c.id}
                <button class="btn btn-primary btn-sm" on:click={() => saveEdit(c.id)}>Guardar</button>
                <button class="btn btn-ghost btn-sm" on:click={cancelEdit}>Cancelar</button>
              {:else}
                <button class="btn btn-ghost btn-sm" on:click={() => startEdit(c)}>Editar</button>
                <button class="btn btn-danger btn-sm" on:click={() => deleteCategory(c.id)}>Eliminar</button>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <!-- Mobile cards -->
  <div class="orders-cards">
    {#each categories as c (c.id)}
      <dl class="order-card">
        <dt>Nombre</dt>
        <dd class="fw-semibold">
          {#if editingId === c.id}
            <input type="text" maxlength="100" bind:value={editName} />
          {:else}
            {c.name}
          {/if}
        </dd>
        <dt>Acciones</dt>
        <dd class="actions-wrap">
          {#if editingId === c.id}
            <button class="btn btn-primary btn-sm" on:click={() => saveEdit(c.id)}>Guardar</button>
            <button class="btn btn-ghost btn-sm" on:click={cancelEdit}>Cancelar</button>
          {:else}
            <button class="btn btn-ghost btn-sm" on:click={() => startEdit(c)}>Editar</button>
            <button class="btn btn-danger btn-sm" on:click={() => deleteCategory(c.id)}>Eliminar</button>
          {/if}
        </dd>
      </dl>
    {/each}
  </div>
{/if}
