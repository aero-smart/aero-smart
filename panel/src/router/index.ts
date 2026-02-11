import { createRouter, createWebHistory } from 'vue-router'
import MainLayout from '@/components/layout/MainLayout.vue'
import DashboardView from '@/views/DashboardView.vue'
import ControlView from '@/views/ControlView.vue'

import PowerView from '@/views/PowerView.vue'
import OnboardingView from '@/views/OnboardingView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/onboarding',
      name: 'onboarding',
      component: OnboardingView,
    },
    {
      path: '/',
      component: MainLayout,
      children: [
        {
          path: '',
          name: 'dashboard',
          component: DashboardView,
        },
        {
          path: 'control',
          name: 'control',
          component: ControlView,
        },
        {
          path: 'analysis',
          name: 'analysis',
          component: PowerView,
        },
        {
          path: 'settings',
          name: 'settings',
          component: () => import('@/views/DashboardView.vue'), // Placeholder
        },
      ],
    },
  ],
})

export default router
