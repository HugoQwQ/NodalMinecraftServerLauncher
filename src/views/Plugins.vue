<template>
  <div class="plugins">
    <div class="header">
      <h1>插件管理</h1>
      <div class="header-actions">
        <div class="search-box">
          <span class="material-icons">search</span>
          <input type="text" placeholder="搜索插件..." />
        </div>
        <button class="btn btn-primary">
          <span class="material-icons">upload</span>
          上传插件
        </button>
      </div>
    </div>

    <div class="plugins-grid">
      <div class="card plugin-card" v-for="i in 4" :key="i">
        <div class="plugin-header">
          <img src="https://via.placeholder.com/50" alt="Plugin Icon" class="plugin-icon" />
          <div class="plugin-info">
            <h3>插件 {{ i }}</h3>
            <span class="version">v1.0.{{ i }}</span>
          </div>
          <label class="toggle">
            <input type="checkbox" checked />
            <span class="toggle-slider"></span>
          </label>
        </div>
        <p class="plugin-description">这是一个示例插件描述，展示插件的主要功能和特点。</p>
        <div class="plugin-actions">
          <button class="btn">
            <i class="fas fa-gear"></i>
            配置
          </button>
          <button class="btn">
            <i class="fas fa-trash"></i>
            删除
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.plugins {
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

.header-actions {
  display: flex;
  gap: var(--space-md);
  margin-top: 0;
}

.search-box {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background-color: var(--card-bg);
  padding: 0.5rem 1rem;
  border-radius: 6px;
  flex: 1;
}

.search-box input {
  border: none;
  background: none;
  outline: none;
  color: var(--text-color);
  width: 100%;
}

.plugins-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
}

.plugin-card {
  display: flex;
  flex-direction: column;
}

.plugin-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1rem;
}

.plugin-icon {
  width: 50px;
  height: 50px;
  border-radius: 8px;
}

.plugin-info {
  flex: 1;
}

.version {
  font-size: 0.875rem;
  color: var(--text-color);
  opacity: 0.7;
}

.plugin-description {
  margin: 1rem 0;
  flex: 1;
}

.plugin-actions {
  display: flex;
  gap: 1rem;
}

.plugin-actions .btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}
</style> 