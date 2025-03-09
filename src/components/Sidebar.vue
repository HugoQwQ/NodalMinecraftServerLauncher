<template>
  <div :class="['sidebar', { 'sidebar-collapsed': isCollapsed }]">
    <div class="sidebar-header">
      <img src="../assets/icon.png" alt="Logo" class="logo" :class="{ 'logo-small': isCollapsed }" />
      <button class="collapse-btn" @click="toggleCollapse">
        <i :class="['fas', isCollapsed ? 'fa-chevron-right' : 'fa-chevron-left']"></i>
      </button>
    </div>

    <nav class="sidebar-nav">
      <router-link
        v-for="item in menuItems"
        :key="item.path"
        :to="item.path"
        class="nav-item"
        :class="{ active: currentPath === item.path }"
      >
        <i :class="['fas', item.icon]"></i>
        <span class="nav-text" v-show="!isCollapsed">{{ item.name }}</span>
      </router-link>
    </nav>

    <div class="sidebar-footer" v-show="!isCollapsed">
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRoute } from 'vue-router'

const props = defineProps<{
  modelValue?: boolean
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

const route = useRoute()
const isCollapsed = ref(props.modelValue ?? false)

const menuItems = [
  { name: '仪表盘', path: '/', icon: 'fa-tachometer-alt' },
  { name: '服务器', path: '/servers', icon: 'fa-server' },
  { name: '插件', path: '/plugins', icon: 'fa-puzzle-piece' },
  { name: '设置', path: '/settings', icon: 'fa-cog' }
]

const currentPath = computed(() => route.path)

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
  emit('update:modelValue', isCollapsed.value)
}

watch(() => props.modelValue, (newValue) => {
  if (newValue !== undefined) {
    isCollapsed.value = newValue
  }
})
</script>

<style scoped>
.sidebar {
  width: 250px;
  background-color: var(--sidebar-bg);
  border-right: 1px solid var(--border-color);
  height: 100vh;
  position: fixed;
  left: 0;
  top: 0;
  transition: width var(--transition-speed) ease;
  display: flex;
  flex-direction: column;
  z-index: 1000;
  overflow: hidden;
}

.sidebar-collapsed {
  width: 100px;
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding: 1rem;
  border-bottom: 1px solid var(--border-color);
  min-height: 64px;
}

.sidebar-collapsed .sidebar-header {
  justify-content: center;
  padding: 0.5rem 1rem;
}

.logo {
  height: 40px;
  transition: all var(--transition-speed) ease;
  margin-right: auto;
}

.logo-small {
  height: 30px;
}

.collapse-btn {
  background: none;
  border: none;
  color: var(--text-color);
  cursor: pointer;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  transition: background-color var(--transition-speed) ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.collapse-btn:hover {
  background-color: rgba(0, 0, 0, 0.1);
}

.sidebar-nav {
  padding: 0.5rem 0.25rem;
  flex: 1;
  overflow-y: auto;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 0.75rem 1rem;
  color: var(--text-color);
  text-decoration: none;
  transition: all var(--transition-speed) ease;
  border-radius: 8px;
  margin: 0.25rem;
  white-space: nowrap;
  min-height: 48px;
  width: calc(100% - 0.5rem);
}

.nav-item:hover {
  background-color: rgba(var(--primary-color-rgb), 0.1);
}

.nav-item.active {
  background-color: var(--primary-color);
  color: white;
}

.nav-text {
  margin-left: 1rem;
  opacity: 1;
  transition: opacity var(--transition-speed) ease;
  white-space: nowrap;
}

.sidebar-collapsed .nav-text {
  opacity: 0;
  width: 0;
}

.sidebar-collapsed .nav-item {
  justify-content: center;
  padding: 0.75rem;
  margin: 0.25rem;
  width: calc(100% - 0.5rem);
}

.sidebar-collapsed .nav-item i {
  margin: 0;
  font-size: 1.3rem;
}

i {
  font-size: 1.2rem;
  min-width: 24px;
  text-align: center;
}

.sidebar-footer {
  padding: 1rem;
  border-top: 1px solid var(--border-color);
  min-height: 64px;
}
</style> 