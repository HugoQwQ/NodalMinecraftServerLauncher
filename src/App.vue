<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import Sidebar from './components/Sidebar.vue'

const isCollapsed = ref(true)

const handleContextMenu = (e: MouseEvent) => {
  e.preventDefault()
}

onMounted(() => {
  document.addEventListener('contextmenu', handleContextMenu)
})

onUnmounted(() => {
  document.removeEventListener('contextmenu', handleContextMenu)
})

</script>

<template>
  <div class="app-container">
    <Sidebar v-model="isCollapsed" />
    <main :class="['main-content', { 'main-content-expanded': isCollapsed }]">
      <router-view></router-view>
    </main>
  </div>
</template>

<style>
@import './app.css';

.app-container {
  display: flex;
  min-height: 100vh;
  overflow-x: hidden;
}

.main-content {
  flex: 1;
  margin-left: 250px;
  padding: 2rem;
  transition: margin-left var(--transition-speed) ease;
  width: calc(100% - 250px);
}

.main-content-expanded {
  margin-left: 90px;
  width: calc(100% - 90px);
}
</style>