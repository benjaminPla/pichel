<script>
  import { onMount } from 'svelte';
  import { apiFetch } from '$lib/api.js';
  import { toast } from '$lib/toast.js';
  import { topbarTitle } from '$lib/adminStore.js';

  topbarTitle.set('Usuarios');

  const PER_PAGE = 50;

  let users = [];
  let total = 0;
  let currentPage = 1;
  let totalPages = 1;
  let loading = true;

  let editingId = null;
  let editingMode = null;
  let editInput = '';
  let editSaving = false;

  onMount(() => loadUsers(1));

  async function loadUsers(page) {
    currentPage = page;
    loading = true;
    const res = await apiFetch(`/users?page=${page}&per_page=${PER_PAGE}`);
    if (!res || !res.ok) { loading = false; return; }
    const data = await res.json();
    users = data.users;
    total = data.total;
    totalPages = Math.ceil(total / PER_PAGE);
    loading = false;
  }

  function openEditEmail(id, email) { editingId = id; editingMode = 'email'; editInput = email; }
  function openEditPassword(id)      { editingId = id; editingMode = 'password'; editInput = ''; }
  function closeEdit()               { editingId = null; editingMode = null; editInput = ''; }

  async function saveEdit(e) {
    e.preventDefault();
    if (!editInput) return;
    editSaving = true;
    try {
      let res;
      if (editingMode === 'email') {
        res = await apiFetch(`/users/${editingId}`, { method: 'PATCH', body: JSON.stringify({ email: editInput }) });
        if (!res || !res.ok) { toast('Error al actualizar el email', 'error'); return; }
        toast('Email actualizado');
      } else {
        res = await apiFetch(`/users/${editingId}/password`, { method: 'PATCH', body: JSON.stringify({ password: editInput }) });
        if (!res || !res.ok) { toast('Error al actualizar la contraseña', 'error'); return; }
        toast('Contraseña actualizada');
      }
      closeEdit();
      await loadUsers(currentPage);
    } catch { toast('Error del servidor', 'error'); }
    finally   { editSaving = false; }
  }

  async function deleteUser(id) {
    if (!confirm('¿Eliminar este usuario?')) return;
    const res = await apiFetch(`/users/${id}`, { method: 'DELETE' });
    if (!res || !res.ok) { toast('Error al eliminar', 'error'); return; }
    toast('Usuario eliminado');
    await loadUsers(currentPage);
  }
</script>

<svelte:head>
  <title>Usuarios — Pichel Admin</title>
</svelte:head>

{#if editingId}
  <div class="form-card form-card--edit">
    <h2 class="form-title">{editingMode === 'email' ? 'Cambiar email' : 'Cambiar contraseña'}</h2>
    <form class="form-stack" on:submit={saveEdit}>
      <div>
        <label for="edit-input">
          {editingMode === 'email' ? 'Nuevo email *' : 'Nueva contraseña *'}
        </label>
        <input id="edit-input"
          type={editingMode === 'email' ? 'email' : 'password'}
          required bind:value={editInput} />
      </div>
      <div class="form-actions">
        <button class="btn btn-primary" type="submit" disabled={editSaving}>
          {editSaving ? '…' : editingMode === 'email' ? 'Guardar email' : 'Guardar contraseña'}
        </button>
        <button class="btn btn-ghost" type="button" on:click={closeEdit}>Cancelar</button>
      </div>
    </form>
  </div>
{/if}

<div class="section-header section-header--flush">
  <h2>Todos los usuarios</h2>
  <div style="display:flex; align-items:center; gap:var(--sp-3)">
    {#if total}<span class="text-muted">{total} usuarios</span>{/if}
    <a href="/admin/users/create" class="btn btn-primary btn-sm">+ Agregar</a>
  </div>
</div>

{#if loading}
  <div class="table-wrap"><p class="table-empty">Cargando…</p></div>
{:else if !users.length}
  <div class="table-wrap"><p class="table-empty">Sin usuarios todavía.</p></div>
{:else}
  <!-- Desktop table -->
  <div class="table-wrap orders-table">
    <table>
      <thead><tr><th>Email</th><th class="nowrap">Acciones</th></tr></thead>
      <tbody>
        {#each users as u (u.id)}
          <tr>
            <td class="td-strong">{u.email}</td>
            <td class="nowrap">
              <div class="actions-wrap">
                <button class="btn btn-ghost btn-sm" on:click={() => openEditEmail(u.id, u.email)}>Cambiar email</button>
                <button class="btn btn-ghost btn-sm" on:click={() => openEditPassword(u.id)}>Cambiar contraseña</button>
                <button class="btn btn-danger btn-sm" on:click={() => deleteUser(u.id)}>Eliminar</button>
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <!-- Mobile cards -->
  <div class="orders-cards">
    {#each users as u (u.id)}
      <dl class="order-card">
        <dt>Email</dt>
        <dd class="fw-semibold">{u.email}</dd>

        <dt>Acciones</dt>
        <dd class="actions-wrap">
          <button class="btn btn-ghost btn-sm" on:click={() => openEditEmail(u.id, u.email)}>Cambiar email</button>
          <button class="btn btn-ghost btn-sm" on:click={() => openEditPassword(u.id)}>Cambiar contraseña</button>
          <button class="btn btn-danger btn-sm" on:click={() => deleteUser(u.id)}>Eliminar</button>
        </dd>
      </dl>
    {/each}
  </div>

  {#if totalPages > 1}
    <div class="pagination">
      <button class="pagination-btn" disabled={currentPage <= 1}
        on:click={() => loadUsers(currentPage - 1)}>← Anterior</button>
      <span class="pagination-info">Página {currentPage} de {totalPages}</span>
      <button class="pagination-btn" disabled={currentPage >= totalPages}
        on:click={() => loadUsers(currentPage + 1)}>Siguiente →</button>
    </div>
  {/if}
{/if}
