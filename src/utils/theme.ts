import { ref, watch } from 'vue'

type Theme = 'light' | 'dark' | 'system'

const theme = ref<Theme>(localStorage.getItem('theme') as Theme || 'system')
const systemDark = ref(window.matchMedia('(prefers-color-scheme: dark)').matches)
const accentColor = ref(localStorage.getItem('accentColor') || '#0078D4')
const followSystemAccent = ref(localStorage.getItem('followSystemAccent') === 'true')

// 监听系统主题变化
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
mediaQuery.addEventListener('change', (e) => {
  systemDark.value = e.matches
  if (theme.value === 'system') {
    applyTheme(e.matches ? 'dark' : 'light')
  }
})

// 应用主题
const applyTheme = (mode: 'light' | 'dark') => {
  document.documentElement.setAttribute('data-theme', mode)
}

// 应用主题色
const applyAccentColor = (color: string) => {
  document.documentElement.style.setProperty('--accent-color', color)
  // 转换颜色为RGB格式
  const rgb = hexToRgb(color)
  if (rgb) {
    document.documentElement.style.setProperty('--accent-color-rgb', `${rgb.r}, ${rgb.g}, ${rgb.b}`)
  }
}

// 十六进制颜色转RGB
const hexToRgb = (hex: string) => {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
  return result ? {
    r: parseInt(result[1], 16),
    g: parseInt(result[2], 16),
    b: parseInt(result[3], 16)
  } : null
}

// 监听主题变化
watch(theme, (newTheme) => {
  localStorage.setItem('theme', newTheme)
  if (newTheme === 'system') {
    applyTheme(systemDark.value ? 'dark' : 'light')
  } else {
    applyTheme(newTheme)
  }
}, { immediate: true })

// 监听主题色变化
watch(accentColor, (newColor) => {
  localStorage.setItem('accentColor', newColor)
  applyAccentColor(newColor)
}, { immediate: true })

// 监听系统主题色跟随设置
watch(followSystemAccent, (newValue) => {
  localStorage.setItem('followSystemAccent', newValue.toString())
}, { immediate: true })

export const useTheme = () => {
  return {
    theme,
    setTheme: (newTheme: Theme) => {
      theme.value = newTheme
    },
    setAccentColor: (color: string) => {
      if (!followSystemAccent.value) {
        accentColor.value = color
      }
    },
    toggleSystemAccent: (value: boolean) => {
      followSystemAccent.value = value
    }
  }
} 