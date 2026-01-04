import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'editor',
    component: () => import('@/components/EditorCanvas.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
