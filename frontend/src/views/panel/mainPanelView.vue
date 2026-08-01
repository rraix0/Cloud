<script setup lang="ts">
import sidebarComponent from './components/sidebarComponent.vue';
import router from '@/router'
import { useRoute } from 'vue-router'

const route = useRoute();

console.log(route.params.token)

if (!(localStorage.getItem('jwt') || route.params.token)) {
  router.push('/');
}

</script>

<template>
  <div class="flex h-screen bg-background text-foreground">
    <sidebarComponent />
    <RouterView />
  </div>
</template>