<script setup lang="ts">
import NumberComponent from '@/components/ClockComponent/NumberComponent.vue'
import { ref } from 'vue'
import './style.scss';

const hours = ref(0)
const minutes = ref(0)
const seconds = ref(0)

function myFunction() {
  const date = new Date()
  hours.value = date.getHours()
  minutes.value = date.getMinutes()
  seconds.value = date.getSeconds()
}

const intervalId = setInterval(myFunction, 1000)

</script>

<template>
  <div class="clock">
    <div class="part">
        <NumberComponent :value="Math.floor(hours / 10)"></NumberComponent>
        <NumberComponent :value="hours % 10"></NumberComponent>
    </div>
    <div class="part">
        <NumberComponent :value="Math.floor(minutes / 10)"></NumberComponent>
        <NumberComponent :value="minutes % 10"></NumberComponent>
    </div>
      <div class="part">
        <NumberComponent :value="Math.floor(seconds / 10)"></NumberComponent>
        <NumberComponent :value="seconds % 10"></NumberComponent>
      </div>
  </div>
</template>

<style lang="scss" scoped>
  .clock {
    width: min-content;
    background-color: var(--component-background-color);
    display: flex;
    gap: var(--digit-spacing);
    padding: var(--component-padding);
    margin: var(--component-margin);
    border-radius: var(--component-border-radius);
    border: var(--component-border);
  }
  .part {
    display: flex;
  }
</style>
