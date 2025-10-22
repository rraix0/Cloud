<script setup lang="ts">
import sidebarComponent from './components/sidebarComponent.vue';
import router from '@/router'

if (!localStorage.getItem('jwt')) {
  router.push('/');
}

</script>

<template>
  <div class="flex min-h-screen bg-background text-foreground">
    <sidebarComponent />
    <RouterView />
  </div>
</template>