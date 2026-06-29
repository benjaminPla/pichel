<script>
  import { goto } from '$app/navigation';
  import { apiFetch } from '$lib/api.js';
  import { toast } from '$lib/toast.js';
  import { topbarTitle } from '$lib/adminStore.js';

  topbarTitle.set('Agregar usuario');

  let email = '';
  let password = '';
  let submitting = false;

  async function submit(e) {
    e.preventDefault();
    if (!email || !password) return;
    submitting = true;
    try {
      const res = await apiFetch('/users', {
        method: 'POST',
        body: JSON.stringify({ email, password }),
      });
      if (!res) return;
      if (!res.ok) { toast('Error al crear el usuario', 'error'); return; }
      goto('/admin/users');
    } catch { toast('Error del servidor', 'error'); }
    finally   { submitting = false; }
  }
</script>

<svelte:head>
  <title>Nuevo usuario — Pichel Admin</title>
</svelte:head>

<div class="form-card">
  <form class="form-stack" on:submit={submit}>
    <div>
      <label for="u-email">Email *</label>
      <input id="u-email" type="email" required bind:value={email} />
    </div>
    <div>
      <label for="u-password">Contraseña *</label>
      <input id="u-password" type="password" required bind:value={password} />
    </div>
    <div class="form-actions">
      <button class="btn btn-primary" type="submit" disabled={submitting}>
        {submitting ? '…' : 'Agregar usuario'}
      </button>
      <a class="btn btn-ghost" href="/admin/users">Cancelar</a>
    </div>
  </form>
</div>
