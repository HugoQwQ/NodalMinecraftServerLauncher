<template>
  <div class="settings">
    <div class="header">
      <h1>设置</h1>
    </div>
    
    <div v-for="section in settings" :key="section.name" class="settings-section card">
      <h2>{{ section.name }}</h2>
      <div v-for="item in section.items" :key="item.name" class="setting-item">
        <div class="setting-info">
          <h3>{{ item.name }}</h3>
          <p>{{ item.description }}</p>
        </div>
        
        <!-- 布尔类型设置项 -->
        <label v-if="item.type === 'boolean'" class="toggle">
          <input 
            type="checkbox" 
            v-model="settingsData[item.name]" 
            @change="(e) => handleSettingChange(item, (e.target as HTMLInputElement).checked)"
          />
          <span class="toggle-slider"></span>
        </label>

        <!-- 下拉选择设置项 -->
        <select 
          v-if="item.type === 'combo'"
          v-model="settingsData[item.name]"
          @change="(e) => handleSettingChange(item, (e.target as HTMLSelectElement).value)"
          class="combo-select"
        >
          <option v-for="option in item.options" :key="option" :value="option">
            {{ option }}
          </option>
        </select>

        <!-- 滑块设置项 -->
        <div v-if="item.type === 'slider'" class="slider-container">
          <input 
            type="range"
            v-model.number="settingsData[item.name]"
            :min="(item as SliderSetting).min"
            :max="(item as SliderSetting).max"
            :step="(item as SliderSetting).step"
            @change="(e) => handleSettingChange(item, Number((e.target as HTMLInputElement).value))"
          />
          <span class="slider-value">{{ settingsData[item.name] }}</span>
        </div>

        <!-- 颜色选择器 -->
        <div v-if="item.type === 'color'" class="color-picker">
          <input 
            type="color" 
            v-model="settingsData[item.name]"
            @change="(e) => handleSettingChange(item, (e.target as HTMLInputElement).value)"
          />
        </div>

        <!-- 按钮组 -->
        <div v-if="item.type === 'button'" class="button-group">
          <button
            v-for="option in item.options"
            :key="option"
            class="btn"
            :class="{ active: settingsData[item.name] === option }"
            @click="handleSettingChange(item, option)"
          >
            {{ option }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import settingsConfig from '../assets/settings.json'
import { useTheme } from '../utils/theme'
import { invoke } from '@tauri-apps/api/tauri'

interface BaseSetting {
  name: string;
  description: string;
  type: 'slider' | 'color' | 'button' | 'combo' | 'boolean';
  default: any;
  options?: string[];
}

interface SliderSetting extends BaseSetting {
  type: 'slider';
  min: number;
  max: number;
  step: number;
  default: number;
}

interface ColorSetting extends BaseSetting {
  type: 'color';
  default: string;
}

interface ButtonSetting extends BaseSetting {
  type: 'button';
  options: string[];
  default: string;
}

interface ComboSetting extends BaseSetting {
  type: 'combo';
  options: string[];
  default: string;
}

interface BooleanSetting extends BaseSetting {
  type: 'boolean';
  default: boolean;
}

type Setting = SliderSetting | ColorSetting | ButtonSetting | ComboSetting | BooleanSetting;

interface SettingsSection {
  name: string;
  items: Setting[];
}

const { theme, setTheme, setAccentColor, toggleSystemAccent } = useTheme()

// 计算当前主题模式
const currentTheme = computed(() => {
  switch (theme.value) {
    case 'dark':
      return '深色'
    case 'light':
      return '浅色'
    default:
      return '跟随系统'
  }
})

const settings = ref<SettingsSection[]>(settingsConfig.settings as SettingsSection[])
const settingsData = ref<Record<string, any>>({})

const handleThemeChange = async (value: string) => {
  let themeValue: string
  switch (value) {
    case '深色':
      themeValue = 'dark'
      setTheme('dark')
      break
    case '浅色':
      themeValue = 'light'
      setTheme('light')
      break
    default:
      themeValue = 'system'
      setTheme('system')
  }
  
  try {
    await invoke('save_config_command', {
      section: 'theme',
      key: 'theme',
      value: themeValue
    })
  } catch (error) {
    console.error('保存主题设置失败:', error)
  }
}

const handleSettingChange = async (item: Setting, value: any) => {
  settingsData.value[item.name] = value
  
  try {
    // 特殊处理主题相关设置
    switch (item.name) {
      case '主题模式':
        await handleThemeChange(value)
        break
      case '主题颜色':
        setAccentColor(value)
        await invoke('save_config_command', {
          section: 'theme',
          key: 'accent_color',
          value
        })
        break
      case '跟随系统主题色':
        toggleSystemAccent(value)
        await invoke('save_config_command', {
          section: 'theme',
          key: 'follow_system_accent',
          value: value.toString()
        })
        break
      case '自动启动':
        await invoke('save_config_command', {
          section: 'normal',
          key: 'autostartup',
          value: value.toString()
        })
        break
      case '控制台编码':
        await invoke('save_config_command', {
          section: 'console',
          key: 'encoding',
          value
        })
        break
      case '自动同意EULA':
        await invoke('save_config_command', {
          section: 'instance',
          key: 'auto_accept_eula',
          value: value.toString()
        })
        break
      case '崩溃自动重启':
        await invoke('save_config_command', {
          section: 'instance',
          key: 'auto_restart_on_crash',
          value: value.toString()
        })
        break
      case '下载源':
        await invoke('save_config_command', {
          section: 'download',
          key: 'source',
          value
        })
        break
      case 'Aria2 下载':
        await invoke('save_config_command', {
          section: 'download',
          key: 'aria2_enabled',
          value: value.toString()
        })
        break
      case 'Aria2 线程数':
        await invoke('save_config_command', {
          section: 'download',
          key: 'aria2_threads',
          value: value.toString()
        })
        break
    }
  } catch (error) {
    console.error('保存设置失败:', error)
  }
  
  saveSettings()
}

// 初始化设置数据
onMounted(async () => {
  try {
    // 从config.toml加载配置
    const config = await invoke('load_config_command')
    
    // 设置主题
    const themeMode = config.theme.theme
    switch (themeMode) {
      case 'dark':
        settingsData.value['主题模式'] = '深色'
        setTheme('dark')
        break
      case 'light':
        settingsData.value['主题模式'] = '浅色'
        setTheme('light')
        break
      default:
        settingsData.value['主题模式'] = '跟随系统'
        setTheme('system')
    }
    
    // 设置主题色
    const accentColor = config.theme.accent_color
    settingsData.value['主题颜色'] = accentColor
    setAccentColor(accentColor)
    
    // 设置系统主题色跟随
    const followSystemAccent = config.theme.follow_system_accent
    settingsData.value['跟随系统主题色'] = followSystemAccent
    toggleSystemAccent(followSystemAccent)
    
    // 设置其他配置
    settingsData.value['自动启动'] = config.normal.autostartup
    settingsData.value['控制台编码'] = config.console.encoding
    settingsData.value['自动同意EULA'] = config.instance.auto_accept_eula
    settingsData.value['崩溃自动重启'] = config.instance.auto_restart_on_crash
    settingsData.value['下载源'] = config.download.source
    settingsData.value['Aria2 下载'] = config.download.aria2_enabled
    settingsData.value['Aria2 线程数'] = config.download.aria2_threads
    
  } catch (error) {
    console.error('加载配置失败:', error)
    
    // 如果加载失败，使用默认值
    settings.value.forEach(section => {
      section.items.forEach(item => {
        if (item.name === '主题模式') {
          settingsData.value[item.name] = currentTheme.value
        } else {
          settingsData.value[item.name] = item.default
        }
      })
    })
  }
})

// 保存到localStorage作为备份
const saveSettings = () => {
  localStorage.setItem('app-settings', JSON.stringify(settingsData.value))
}

// const autoStart = ref(false)
// const acceptEULA = ref(false)
// const accentColor = ref('#0078D4')
// const updateAccentColor = () => {
//   setAccentColor(accentColor.value)
// }

// const openDevTools = async () => {
//   await invoke('open_devtools')
// }
</script>

<style scoped>
.settings {
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

.settings-section {
  margin-bottom: var(--space-xl);
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

.theme-control {
  flex-direction: column;
  align-items: flex-end;
  gap: var(--space-md);
}

.theme-buttons {
  display: flex;
  gap: var(--space-sm);
}

.theme-buttons .btn {
  min-width: auto;
  padding: var(--space-sm) var(--space-md);
}

.theme-buttons .btn.active {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.theme-buttons .btn i {
  margin-right: var(--space-xs);
}

.theme-color {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
  align-items: flex-start;
}

.color-picker {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
}

.color-picker input {
  width: 40px;
  height: 40px;
  padding: 0;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.checkbox {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  cursor: pointer;
}

.checkbox input {
  width: 16px;
  height: 16px;
}

.combo-select {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--bg-color);
  color: var(--text-color);
  min-width: 200px;
}

.slider-container {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.slider-container input[type="range"] {
  width: 200px;
}

.slider-value {
  min-width: 40px;
  text-align: right;
}

.button-group {
  display: flex;
  gap: 0.5rem;
}

.button-group .btn {
  padding: 0.5rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  background: var(--bg-color);
  color: var(--text-color);
  cursor: pointer;
  transition: all 0.2s;
}

.button-group .btn.active {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.toggle {
  position: relative;
  display: inline-block;
  width: 52px;
  height: 28px;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--border-color);
  transition: .4s;
  border-radius: 34px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 20px;
  width: 20px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: .4s;
  border-radius: 50%;
}

input:checked + .toggle-slider {
  background-color: var(--accent-color);
}

input:checked + .toggle-slider:before {
  transform: translateX(24px);
}
</style> 