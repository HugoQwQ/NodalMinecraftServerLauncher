<template>
  <div class="dashboard">
    <div class="header">
      <h1>仪表盘</h1>
    </div>
    <div class="dashboard-grid">
      <div class="card">
        <h3>服务器状态</h3>
        <div class="status-info">
          <i class="fas fa-memory"></i>
          <div class="info">
            <p>在线服务器：{{ onlineServers }}</p>
            <p>总服务器：{{ totalServers }}</p>
          </div>
        </div>
      </div>
      
      <div class="card">
        <h3>系统资源</h3>
        <div class="resource-info">
          <div class="resource-item">
            <span>CPU ({{ systemResources.cpu.cores }} 核心)</span>
            <div class="progress-bar">
              <div class="progress" :style="{ width: `${systemResources.cpu.usage}%` }"></div>
            </div>
          </div>
          <div class="resource-item">
            <span>内存 ({{ formatMemory(systemResources.memory.used) }}/{{ formatMemory(systemResources.memory.total) }})</span>
            <div class="progress-bar">
              <div class="progress" :style="{ width: `${(systemResources.memory.used / systemResources.memory.total) * 100}%` }"></div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="card">
        <h3>公告</h3>
        <div class="status-info">
          <i class="fas fa-bullhorn"></i>
          <div class="info">
            <p>NMSL，启动！</p>
            <br>
            <p>软件交流群：</p>
            <p>一群：1234567890</p>
            <p>二群：1234567890</p>
          </div>
        </div>
      </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { SystemResources } from '../types'

// 模拟数据
const systemResources = ref<SystemResources>({
  cpu: {
    usage: 45,
    cores: 8
  },
  memory: {
    total: 16 * 1024 * 1024 * 1024, // 16GB
    used: 8 * 1024 * 1024 * 1024,   // 8GB
    free: 8 * 1024 * 1024 * 1024    // 8GB
  },
  disk: {
    total: 512 * 1024 * 1024 * 1024,
    used: 256 * 1024 * 1024 * 1024,
    free: 256 * 1024 * 1024 * 1024
  }
})

const onlineServers = ref(3)
const totalServers = ref(5)

const formatMemory = (bytes: number): string => {
  const gb = bytes / (1024 * 1024 * 1024)
  return `${gb.toFixed(1)}GB`
}
</script>

<style scoped>
.dashboard {
  padding: 1rem;

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .dashboard-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
  }
}

.status-info {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-top: 1rem;
}

.fas {
  font-size: 1.5rem;
  color: var(--primary-color);
}

.resource-info {
  margin-top: 1rem;
}

.resource-item {
  margin-bottom: 1rem;
}

.progress-bar {
  height: 8px;
  background-color: var(--border-color);
  border-radius: 4px;
  margin-top: 0.5rem;
}

.progress {
  height: 100%;
  background-color: var(--primary-color);
  border-radius: 4px;
  transition: width 0.3s ease;
}
</style> 