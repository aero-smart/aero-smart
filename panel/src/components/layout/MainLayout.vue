<template>
  <div class="flex h-screen w-screen bg-background text-slate-200 overflow-hidden font-sans selection:bg-primary selection:text-white">
    <Sidebar />
    <main class="flex-1 flex flex-col h-full overflow-hidden relative">
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" class="h-full w-full" />
        </transition>
      </router-view>
    </main>
  </div>
</template>

<script setup lang="ts">
import Sidebar from './Sidebar.vue';
</script>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
