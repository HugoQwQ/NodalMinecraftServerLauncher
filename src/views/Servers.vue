<template>
  <div class="servers">
    <div class="header">
      <h1>服务器管理</h1>
      <div class="header-actions">
        <button class="btn btn-icon" @click="refreshServers">
          <i class="fas fa-sync-alt"></i>
        </button>
        <button class="btn btn-primary" @click="handleAddServer">
          <i class="fas fa-plus"></i>
          添加实例
        </button>
      </div>
    </div>

    <div class="server-grid">
      <div class="card server-card" v-for="server in servers" :key="server.id">
        <div class="server-header">
          <h3>{{ server.name }}</h3>
          <span class="status" :class="server.status">{{ server.status === 'online' ? '在线' : '离线' }}</span>
        </div>
        <div class="server-info">
          <p><i class="fas fa-globe"></i> 服务器端口: {{ server.host }}:{{ server.port }}</p>
          <p><i class="fas fa-users"></i> 在线玩家：{{ server.currentPlayers }}/{{ server.maxPlayers }}</p>
          <p><i class="fas fa-memory"></i> 内存：{{ formatMemory(server.memory.allocated) }}/{{ formatMemory(server.memory.max) }}</p>
        </div>
        <div class="server-actions">
          <button 
            class="btn" 
            :class="server.status === 'online' ? 'stop' : 'start'"
            @click="handleServerAction(server)"
          >
            <i :class="['fas', server.status === 'online' ? 'fa-stop' : 'fa-play']"></i>
            {{ server.status === 'online' ? '停止' : '启动' }}
          </button>
          <button class="btn settings" @click="handleServerSettings(server)">
            <i class="fas fa-cog"></i>
            设置
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { ServerConfig } from '../types'

// 模拟服务器数据
const servers = ref<ServerConfig[]>([
  {
    id: '1',
    name: '生存服务器',
    host: '127.0.0.1',
    port: 25565,
    status: 'online',
    maxPlayers: 20,
    currentPlayers: 10,
    version: '1.19.2',
    memory: {
      max: 4096 * 1024 * 1024,
      allocated: 2048 * 1024 * 1024
    }
  },
  {
    id: '2',
    name: '创造服务器',
    host: '127.0.0.1',
    port: 25566,
    status: 'offline',
    maxPlayers: 10,
    currentPlayers: 0,
    version: '1.19.2',
    memory: {
      max: 2048 * 1024 * 1024,
      allocated: 0
    }
  }
])

const formatMemory = (bytes: number): string => {
  const mb = bytes / (1024 * 1024)
  return `${mb}MB`
}

const refreshServers = async () => {
  // TODO: 实现从后端获取服务器列表的逻辑
  console.log('刷新服务器列表')
}

const handleAddServer = () => {
  // TODO: 实现添加服务器逻辑
  console.log('添加服务器')
}

const handleServerAction = (server: ServerConfig) => {
  // TODO: 实现服务器启动/停止逻辑
  console.log(`${server.status === 'online' ? '停止' : '启动'}服务器`, server.id)
}

const handleServerSettings = (server: ServerConfig) => {
  // TODO: 实现服务器设置逻辑
  console.log('服务器设置', server.id)
}
</script>

<style scoped>
.servers {
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .server-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
  }
}

.server-card {
  display: flex;
  flex-direction: column;
}

.server-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.status {
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.875rem;
}

.status.online {
  background-color: #4caf50;
  color: white;
}

.status.offline {
  background-color: #f44336;
  color: white;
}

.server-info {
  flex: 1;
}

.server-info p {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.fas {
  font-size: 1.2rem;
  color: var(--text-color);
}

.server-actions {
  display: flex;
  gap: 1rem;
  margin-top: 1rem;
}

.server-actions .btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background-color: var(--bg-color);
  border: 1px solid var(--border-color);
  color: var(--text-color);
  transition: all var(--transition-speed) ease;
}

.server-actions .btn:hover {
  background-color: var(--border-color);
}

.server-actions .btn.start {
  border-color: #4caf50;
  color: #4caf50;
}

.server-actions .btn.start:hover {
  background-color: #4caf50;
  color: white;
}

.server-actions .btn.stop {
  border-color: #f44336;
  color: #f44336;
}

.server-actions .btn.stop:hover {
  background-color: #f44336;
  color: white;
}

.server-actions .btn.settings {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.server-actions .btn.settings:hover {
  background-color: var(--primary-color);
  color: white;
}

.btn-icon {
  width: 36px;
  height: 36px;
  padding: 0;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-color);
  border: 1px solid var(--border-color);
  color: var(--text-color);
  transition: all var(--transition-speed) ease;
}

.btn-icon:hover {
  background-color: var(--border-color);
  transform: rotate(180deg);
}

.btn-icon i {
  font-size: 1rem;
}
</style> 