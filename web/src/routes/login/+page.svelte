<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  const API = '';

  let emailVal = '';
  let password = '';
  let errorMsg = '';
  let submitting = false;

  onMount(async () => {
    try {
      const res = await fetch(`${API}/users?page=1&per_page=1`);
      if (res.ok) goto('/admin');
    } catch (_) {}
  });

  async function submit(e) {
    e.preventDefault();
    errorMsg = '';
    if (!emailVal || !password) { errorMsg = 'Email y contraseña son requeridos.'; return; }

    submitting = true;
    try {
      const res = await fetch(`${API}/auth/authenticate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: emailVal, password }),
      });
      if (res.status === 401) { errorMsg = 'Email o contraseña incorrectos.'; return; }
      if (!res.ok)             { errorMsg = `Error del servidor (${res.status}). Intentá de nuevo.`; return; }
      goto('/admin');
    } catch {
      errorMsg = 'No se pudo conectar con el servidor. ¿Está corriendo la API?';
    } finally {
      submitting = false;
    }
  }
</script>

<svelte:head>
  <title>Ingresar — Pichel</title>
</svelte:head>

<div style="min-height:100vh; display:grid; place-items:center; padding:var(--sp-4)">
  <div class="card card-form">
    <a href="/" class="brand">Pichel</a>
    <h1 class="auth-subtitle">Panel de administración</h1>

    {#if errorMsg}
      <div class="error-msg show">{errorMsg}</div>
    {/if}

    <form on:submit={submit} novalidate>
      <div class="field">
        <label for="email">Email</label>
        <input id="email" type="email" bind:value={emailVal}
          placeholder="admin@pichel.com" autocomplete="username" required />
      </div>
      <div class="field">
        <label for="password">Contraseña</label>
        <input id="password" type="password" bind:value={password}
          placeholder="••••••••" autocomplete="current-password" required />
      </div>
      <button class="btn btn-primary btn-block" type="submit" disabled={submitting}>
        {#if submitting}<span class="spinner"></span>Ingresando…{:else}Ingresar{/if}
      </button>
    </form>

    <p class="footer-link"><a href="/">← Volver a la tienda</a></p>
  </div>
</div>
