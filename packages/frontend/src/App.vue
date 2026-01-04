<template>
  <div class="app">
    <header class="app-header">
      <h1>Editor Studio</h1>
      <nav class="app-nav">
        <router-link to="/">编辑器</router-link>
      </nav>
    </header>
    <main class="app-main">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
// Root component
</script>

<style scoped>
.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: #0f0f0f;
  color: #ffffff;
}

.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1.5rem;
  background-color: #1a1a1a;
  border-bottom: 1px solid #2a2a2a;
}

.app-header h1 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
  letter-spacing: -0.025em;
}

.app-nav {
  display: flex;
  gap: 1rem;
}

.app-nav a {
  color: #a0a0a0;
  text-decoration: none;
  font-size: 0.875rem;
  transition: color 0.15s ease;
}

.app-nav a:hover,
.app-nav a.router-link-active {
  color: #ffffff;
}

.app-main {
  flex: 1;
  display: flex;
  flex-direction: column;
}
</style>
