<script>
  import { onMount } from 'svelte';
  import { goto, afterNavigate } from '$app/navigation';
  import { page } from '$app/stores';
  import { toastStore } from '$lib/toast.js';
  import { sidebarOpen, topbarTitle } from '$lib/adminStore.js';

  const API = '';
  let authed = false;

  onMount(async () => {
    const res = await fetch(`${API}/users?page=1&per_page=1`);
    if (res.status === 401) { goto('/login'); return; }
    authed = true;
  });

  afterNavigate(() => { $sidebarOpen = false; });

  async function logout() {
    await fetch(`${API}/auth/logout`, { method: 'POST' });
    goto('/login');
  }

  $: path = $page.url.pathname;
</script>

{#if authed}
  <div style="display:flex; min-height:100vh; width:100%">
    <aside class="sidebar" class:open={$sidebarOpen}>
      <a href="/admin" class="sidebar-brand">Pichel <small>ADMIN</small></a>
      <nav class="sidebar-nav">
        <a class="nav-item" class:active={path === '/admin'} href="/admin">
          <span class="icon">📋</span> Pedidos
        </a>
        <a class="nav-item" class:active={path.startsWith('/admin/products')} href="/admin/products">
          <span class="icon">📦</span> Productos
        </a>
        <a class="nav-item" class:active={path === '/admin/users'} href="/admin/users">
          <span class="icon">👤</span> Usuarios
        </a>
        <a class="nav-item nav-item--push" href="/" target="_blank" rel="noopener">
          <span class="icon">🛒</span> Ver tienda
        </a>
      </nav>
      <div class="sidebar-footer">
        <button class="btn-logout" on:click={logout}>Cerrar sesión</button>
      </div>
    </aside>

    <div class="main">
      <div class="topbar">
        <button class="menu-toggle" aria-label="Abrir menú"
          on:click={() => ($sidebarOpen = !$sidebarOpen)}>
          <span></span><span></span><span></span>
        </button>
        <span class="topbar-title">{$topbarTitle}</span>
      </div>
      <div class="content">
        <slot />
      </div>
    </div>
  </div>

  {#if $toastStore}
    <div class="toast show {$toastStore.type}">{$toastStore.msg}</div>
  {/if}
{/if}
