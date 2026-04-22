import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('@/views/Home.vue'),
    },
    {
      path: '/logs',
      name: 'logs',
      component: () => import('@/views/Logs.vue'),
    },
    {
      path: '/stats',
      name: 'stats',
      component: () => import('@/views/Stats.vue'),
    },
  ],
})

export default router
