import { createRouter, createWebHashHistory } from 'vue-router'
import Dashboard from '../views/Dashboard.vue'
import Servers from '../views/Servers.vue'
import Plugins from '../views/Plugins.vue'
import Settings from '../views/Settings.vue'

const routes = [
  {
    path: '/',
    name: 'Dashboard',
    component: Dashboard,
    meta: {
      title: '仪表盘'
    }
  },
  {
    path: '/servers',
    name: 'Servers',
    component: Servers,
    meta: {
      title: '服务器管理'
    }
  },
  {
    path: '/plugins',
    name: 'Plugins',
    component: Plugins,
    meta: {
      title: '插件管理'
    }
  },
  {
    path: '/settings',
    name: 'Settings',
    component: Settings,
    meta: {
      title: '设置'
    }
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/'
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

router.beforeEach((to, from, next) => {
  document.title = `${to.meta.title || '首页'} - NodalMinecraftServerLauncher`
  next()
})

export default router 