<script>
  import { cartCount } from '$lib/cart.js';

  let navOpen = false;
</script>

<svelte:document on:click={() => (navOpen = false)} />

<nav class="topnav" class:open={navOpen}>
  <a href="/" class="nav-brand">
    <img src="/images/logo.webp" alt="Pichel" height="40" />
  </a>
  <a href="/cart" class="nav-cart" aria-label="Ver carrito" on:click|stopPropagation>
    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="9" cy="21" r="1"/><circle cx="20" cy="21" r="1"/>
      <path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"/>
    </svg>
    {#if $cartCount > 0}
      <span class="nav-cart-badge">{$cartCount}</span>
    {/if}
  </a>
  <button class="nav-hamburger" aria-label="Abrir menú"
    on:click|stopPropagation={() => (navOpen = !navOpen)}>
    <span></span><span></span><span></span>
  </button>
  <ul class="topnav-links">
    <li><a href="/">Precios</a></li>
    <li><a href="/como-comprar">Cómo comprar</a></li>
    <li><a href="/referencias">Referencias</a></li>
    <li><a href="/nosotros">Nosotros</a></li>
    <li><a href="/contacto">Contacto</a></li>
  </ul>
</nav>

<slot />

<footer>
  &copy; {new Date().getFullYear()} Pichel · Almacén Natural
</footer>
