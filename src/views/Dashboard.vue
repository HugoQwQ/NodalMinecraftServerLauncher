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
  padding: var(--space-lg);
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-xl);
}

.header h1 {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
}

.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--space-xl);
  margin-bottom: var(--space-xl);
}

.card {
  background: var(--bg-card);
  border-radius: var(--radius-lg);
  padding: var(--space-lg);
  border: 1px solid var(--border-default);
  box-shadow: var(--elevation-1);
  transition: box-shadow var(--animation-normal);

  &:hover {
    box-shadow: var(--elevation-2);
  }

  h3 {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: var(--space-md);
  }
}

.status-info {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  margin-top: var(--space-md);

  .info {
    p {
      margin: var(--space-xs) 0;
      color: var(--text-secondary);
    }
  }
}

.fas {
  font-size: 24px;
  color: var(--accent-color);
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-secondary);
  border-radius: var(--radius-lg);
}

.resource-info {
  margin-top: var(--space-lg);
}

.resource-item {
  margin-bottom: var(--space-lg);

  span {
    color: var(--text-secondary);
    font-size: 14px;
  }
}

.progress-bar {
  height: 8px;
  background-color: var(--bg-secondary);
  border-radius: var(--radius-sm);
  margin-top: var(--space-xs);
  overflow: hidden;
}

.progress {
  height: 100%;
  background-color: var(--accent-color);
  border-radius: var(--radius-sm);
  transition: width var(--animation-normal);
}
</style> 