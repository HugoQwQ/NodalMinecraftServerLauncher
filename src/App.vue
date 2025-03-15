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
.app-container {
  display: flex;
  min-height: 100vh;
  overflow-x: hidden;
  background-color: var(--bg-default);
}

.main-content {
  flex: 1;
  margin-left: 250px;
  padding: 0;
  transition: all var(--animation-normal);
  width: calc(100% - 250px);
  min-height: 100vh;
  background-color: var(--bg-default);
}

.main-content-expanded {
  margin-left: 60px;
  width: calc(100% - 60px);
}
</style>