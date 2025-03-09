<template>
  <div class="container">
    <div class="tabs">
      <div 
        v-for="tab in tabs" 
        :key="tab.id"
        class="tab"
        :class="{ active: currentTab === tab.id }"
        @click="currentTab = tab.id"
      >
        <i :class="['fas', tab.icon]"></i>
        {{ tab.name }}
      </div>
    </div>

    <div class="content">
      <!-- 概述页面 -->
      <div v-if="currentTab === 'overview'" class="tab-content">
        <div class="dashboard">
          <div class="card dashboard-item">
            <h3>系统状态</h3>
            <div class="stats">
              <div class="stat-item">
                <div class="stat-label">CPU使用率</div>
                <div class="stat-value">
                  <div class="progress-bar">
                    <div class="progress" :style="{ width: '45%' }"></div>
                  </div>
                  <span>45%</span>
                </div>
              </div>
              <div class="stat-item">
                <div class="stat-label">内存使用</div>
                <div class="stat-value">
                  <div class="progress-bar">
                    <div class="progress" :style="{ width: '60%' }"></div>
                  </div>
                  <span>2.4GB / 4GB</span>
                </div>
              </div>
              <div class="stat-item">
                <div class="stat-label">TPS</div>
                <div class="stat-value">20.0</div>
              </div>
            </div>
          </div>

          <div class="card dashboard-item">
            <h3>玩家列表</h3>
            <div class="player-list">
              <div class="player-item">
                <img src="https://mc-heads.net/avatar/Steve/32" alt="Player head" />
                <span>Player1</span>
              </div>
              <div class="player-item">
                <img src="https://mc-heads.net/avatar/Alex/32" alt="Player head" />
                <span>Player2</span>
              </div>
            </div>
          </div>
        </div>

        <div class="card log-viewer">
          <div class="log-header">
            <h3>服务器日志</h3>
            <div class="log-controls">
              <button class="btn btn-icon" title="清除日志">
                <i class="fas fa-trash"></i>
              </button>
              <button class="btn btn-icon" title="滚动到底部">
                <i class="fas fa-arrow-down"></i>
              </button>
              <div class="log-filter">
                <input type="text" placeholder="过滤日志..." />
              </div>
            </div>
          </div>
          <div class="log-content">
            <div class="log-line info">[INFO] 服务器启动中...</div>
            <div class="log-line info">[INFO] 正在加载世界...</div>
            <div class="log-line warn">[WARN] 发现过期模组配置</div>
            <div class="log-line error">[ERROR] 无法加载某个模组</div>
            <div class="log-line info">[INFO] 服务器启动完成</div>
          </div>
        </div>
      </div>

      <!-- 终端页面 -->
      <div v-if="currentTab === 'terminal'" class="tab-content">
        <div class="card terminal">
          <div class="terminal-header">
            <h3>服务器终端</h3>
            <div class="terminal-controls">
              <button class="btn btn-icon" title="清除终端">
                <i class="fas fa-trash"></i>
              </button>
              <button class="btn btn-icon" title="滚动到底部">
                <i class="fas fa-arrow-down"></i>
              </button>
            </div>
          </div>
          <div class="terminal-content">
            <div class="terminal-line">> Starting server...</div>
            <div class="terminal-line">> Loading world data...</div>
            <div class="terminal-line">> Server started on port 25565</div>
          </div>
          <div class="terminal-input">
            <input type="text" placeholder="输入命令..." v-model="terminalInput" @keyup.enter="sendCommand" />
          </div>
        </div>
      </div>

      <!-- 文件管理页面 -->
      <div v-if="currentTab === 'files'" class="tab-content">
        <div class="card file-manager">
          <div class="file-header">
            <div class="breadcrumb">
              <span class="breadcrumb-item">
                <i class="fas fa-home"></i>
              </span>
              <span class="breadcrumb-separator">/</span>
              <span class="breadcrumb-item">mods</span>
            </div>
            <div class="file-controls">
              <button class="btn btn-icon" title="上传文件">
                <i class="fas fa-upload"></i>
              </button>
              <button class="btn btn-icon" title="新建文件夹">
                <i class="fas fa-folder-plus"></i>
              </button>
              <button class="btn btn-icon" title="刷新">
                <i class="fas fa-sync-alt"></i>
              </button>
            </div>
          </div>
          <div class="file-list">
            <div class="file-item folder">
              <i class="fas fa-folder"></i>
              <span class="file-name">config</span>
            </div>
            <div class="file-item folder">
              <i class="fas fa-folder"></i>
              <span class="file-name">mods</span>
            </div>
            <div class="file-item file">
              <i class="fas fa-file"></i>
              <span class="file-name">server.properties</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 设置页面 -->
      <div v-if="currentTab === 'settings'" class="tab-content">
        <div class="card settings">
          <h3>实例设置</h3>
          <div class="settings-grid">
            <div class="setting-item">
              <label>Java路径</label>
              <div class="path-input">
                <input type="text" v-model="javaPath" readonly />
                <button class="btn">
                  <i class="fas fa-folder-open"></i>
                </button>
              </div>
            </div>
            <div class="setting-item">
              <label>最大内存 (MB)</label>
              <div class="memory-input">
                <input type="number" v-model="maxMemory" min="1024" step="1024" />
                <input type="range" v-model="maxMemory" min="1024" max="32768" step="1024" />
              </div>
            </div>
            <div class="setting-item">
              <label>JVM参数</label>
              <textarea v-model="jvmArgs" rows="3"></textarea>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const tabs = [
  { id: 'overview', name: '概述', icon: 'fa-chart-bar' },
  { id: 'terminal', name: '终端', icon: 'fa-terminal' },
  { id: 'files', name: '文件管理', icon: 'fa-folder' },
  { id: 'settings', name: '设置', icon: 'fa-cog' }
]

const currentTab = ref('overview')
const terminalInput = ref('')
const javaPath = ref('C:\\Program Files\\Java\\jdk-17\\bin\\java.exe')
const maxMemory = ref(4096)
const jvmArgs = ref('-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200')

const sendCommand = () => {
  // TODO: 实现发送命令的逻辑
  console.log('发送命令:', terminalInput.value)
  terminalInput.value = ''
}
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.tabs {
  display: flex;
  padding: 0 1rem;
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-color);
}

.tab {
  padding: 1rem 1.5rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  border-bottom: 2px solid transparent;
  transition: all 0.3s ease;
}

.tab:hover {
  color: var(--primary-color);
}

.tab.active {
  color: var(--primary-color);
  border-bottom-color: var(--primary-color);
}

.content {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
}

.tab-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.terminal {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 400px;
}

.terminal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-color);
}

.terminal-content {
  flex: 1;
  overflow-y: auto;
  padding: 1rem 0;
  font-family: monospace;
  background: var(--bg-color);
}

.terminal-line {
  padding: 0.25rem 0;
  color: var(--text-color);
}

.terminal-input {
  padding: 1rem 0;
  border-top: 1px solid var(--border-color);
}

.terminal-input input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-color);
  color: var(--text-color);
  font-family: monospace;
}

.file-manager {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 400px;
}

.file-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-color);
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.breadcrumb-item {
  cursor: pointer;
}

.breadcrumb-item:hover {
  color: var(--primary-color);
}

.breadcrumb-separator {
  color: var(--border-color);
}

.file-controls {
  display: flex;
  gap: 0.5rem;
}

.file-list {
  flex: 1;
  overflow-y: auto;
  padding: 1rem 0;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  cursor: pointer;
  border-radius: 4px;
}

.file-item:hover {
  background: var(--border-color);
}

.file-item i {
  width: 20px;
}

.file-item.folder i {
  color: var(--primary-color);
}

/* 保留原有的其他样式 */
</style> 