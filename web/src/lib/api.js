import { goto } from '$app/navigation';
import { browser } from '$app/environment';

export async function apiFetch(path, opts = {}) {
  const res = await fetch(path, {
    ...opts,
    headers: { 'Content-Type': 'application/json', ...(opts.headers ?? {}) },
  });
  if (res.status === 401) {
    if (browser) goto('/login');
    return null;
  }
  return res;
}

export async function apiUpload(path, formData) {
  const res = await fetch(path, { method: 'POST', body: formData });
  if (res.status === 401) {
    if (browser) goto('/login');
    return null;
  }
  return res;
}
