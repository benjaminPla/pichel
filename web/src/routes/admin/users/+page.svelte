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

  let newEmail = '';
  let newPassword = '';
  let creating = false;

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

  async function createUser(e) {
    e.preventDefault();
    if (!newEmail || !newPassword) return;
    creating = true;
    try {
      const res = await apiFetch('/users', {
        method: 'POST',
        body: JSON.stringify({ email: newEmail, password: newPassword }),
      });
      if (!res || !res.ok) { toast('Error al crear el usuario', 'error'); return; }
      toast('Usuario creado');
      newEmail = '';
      newPassword = '';
      await loadUsers(currentPage);
    } catch { toast('Error del servidor', 'error'); }
    finally   { creating = false; }
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

<div class="form-card">
  <h2 class="form-title">Nuevo usuario</h2>
  <form class="form-stack" on:submit={createUser}>
    <div>
      <label for="u-email">Email *</label>
      <input id="u-email" type="email" required bind:value={newEmail} />
    </div>
    <div>
      <label for="u-password">Contraseña *</label>
      <input id="u-password" type="password" required bind:value={newPassword} />
    </div>
    <div>
      <button class="btn btn-primary" type="submit" disabled={creating}>
        {creating ? '…' : 'Agregar usuario'}
      </button>
    </div>
  </form>
</div>

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
  {#if total}<span class="text-muted">{total} usuarios</span>{/if}
</div>

<div class="table-wrap">
  {#if loading}
    <p class="table-empty">Cargando…</p>
  {:else if !users.length}
    <p class="table-empty">Sin usuarios todavía.</p>
  {:else}
    <table>
      <thead><tr><th>Email</th><th>Acciones</th></tr></thead>
      <tbody>
        {#each users as u (u.id)}
          <tr>
            <td class="td-strong">{u.email}</td>
            <td>
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
</div>
