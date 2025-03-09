<template>
  <div class="settings">
    <h1>设置</h1>
    
    <div class="settings-section card">
      <h2>常规设置</h2>
      <div class="setting-item">
        <div class="setting-info">
          <h3>深色模式</h3>
          <p>切换应用的深色/浅色主题</p>
        </div>
        <label class="toggle">
          <input type="checkbox" v-model="darkMode" />
          <span class="toggle-slider"></span>
        </label>
      </div>
      
      <div class="setting-item">
        <div class="setting-info">
          <h3>自动启动</h3>
          <p>开机时自动启动应用</p>
        </div>
        <label class="toggle">
          <input type="checkbox" v-model="autoStart" />
          <span class="toggle-slider"></span>
        </label>
      </div>
    </div>

    <div class="settings-section card">
      <h2>Java 设置</h2>
      <div class="setting-item">
        <div class="setting-info">
          <h3>Java 路径</h3>
          <p>选择 Java 运行时路径</p>
        </div>
        <div class="setting-control">
          <input type="text" v-model="javaPath" placeholder="请选择 Java 路径" />
          <button class="btn">浏览</button>
        </div>
      </div>

      <div class="setting-item">
        <div class="setting-info">
          <h3>最大内存</h3>
          <p>设置服务器最大内存使用量</p>
        </div>
        <div class="setting-control">
          <input type="number" v-model="maxMemory" min="1024" step="1024" />
          <span>MB</span>
        </div>
      </div>
    </div>
    <div class="settings-section card">
      <h2>开发者设置</h2>
      <div class="setting-item">
        <div class="setting-info">
          <h3>打开开发者工具</h3>
          <p>测试用</p>
        </div>
        <div class="setting-control">
          <button class="btn" @click="openDevTools">打开</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
// import { invoke } from '@tauri-apps/api'

const darkMode = ref(false)
const autoStart = ref(false)
const javaPath = ref('')
const maxMemory = ref(4096)

// const openDevTools = async () => {
//   await invoke('open_devtools')
// }
</script>

<style scoped>
.settings {
  padding: 1rem;
}

.settings-section {
  margin-bottom: 2rem;
}

.settings-section h2 {
  margin-bottom: 1.5rem;
  font-size: 1.25rem;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 0;
  border-bottom: 1px solid var(--border-color);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-info h3 {
  margin-bottom: 0.25rem;
  font-size: 1rem;
}

.setting-info p {
  color: var(--text-color);
  opacity: 0.7;
  font-size: 0.875rem;
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.setting-control input {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--bg-color);
  color: var(--text-color);
}

.setting-control input[type="number"] {
  width: 120px;
}
</style> 