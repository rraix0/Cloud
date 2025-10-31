<script setup lang="ts">
import SmallComponent from '@/components/ClockComponent/SmallComponent.vue'
import { ref } from 'vue'
import { patterns } from './num_pattern.ts'
const props = defineProps(['value'])
import './style.scss';

const cols = ref(4);
const rows = ref(6);

</script>

<template>
  <div class="number-component">
    <div class="number" v-for="x in rows">
      <div class="row" v-for="y in cols">
        <SmallComponent :mode="patterns?.[props.value]?.[x-1]?.[y-1] ?? '/'"></SmallComponent>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.number{
  display: flex;
  flex-direction: row;
}
</style>
