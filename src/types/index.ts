// 服务器配置类型
export interface ServerConfig {
  id: string
  name: string
  host: string
  port: number
  status: 'online' | 'offline'
  maxPlayers: number
  currentPlayers: number
  version: string
  memory: {
    max: number
    allocated: number
  }
}

// 插件类型
export interface Plugin {
  id: string
  name: string
  version: string
  description: string
  enabled: boolean
  icon?: string
  author?: string
  dependencies?: string[]
}

// 系统设置类型
export interface Settings {
  general: {
    darkMode: boolean
    autoStart: boolean
    language: string
  }
  java: {
    path: string
    maxMemory: number
    minMemory: number
    additionalArgs: string[]
  }
  servers: {
    defaultPort: number
    backupLocation: string
    autoRestart: boolean
  }
}

// 系统资源使用情况
export interface SystemResources {
  cpu: {
    usage: number
    cores: number
  }
  memory: {
    total: number
    used: number
    free: number
  }
  disk: {
    total: number
    used: number
    free: number
  }
} 