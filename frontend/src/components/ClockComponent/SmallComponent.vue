<script setup lang="ts">
import { ref, watch } from 'vue'
import './style.scss';

const props = defineProps(['mode'])
const arrow1 = ref(225)
const arrow2 = ref(225)

function updateArrows(mode: string) {
  switch (props.mode) {
    case '━':
      arrow1.value = 90
      arrow2.value = 270
      break
    case '┏':
      arrow1.value = 90
      arrow2.value = 180
      break
    case '┓':
      arrow1.value = 270
      arrow2.value = 180
      break
    case '┗':
      arrow1.value = 0
      arrow2.value = 90
      break
    case '┛':
      arrow1.value = 0
      arrow2.value = 270
      break
    case '┃':
      arrow1.value = 0
      arrow2.value = 180
      break
    case '/':
      arrow1.value = 225
      arrow2.value = 225
  }
}

updateArrows(props.mode)

watch(
  () => props.mode,
  (newMode) => updateArrows(newMode),
)
</script>

<template>
  <div class="small_clock">
    <div class="arrow" :style="{ rotate: arrow1 + 'deg' }"></div>
    <div class="arrow" :style="{ rotate: arrow2 + 'deg' }"></div>
  </div>
</template>

<style scoped>
.small_clock {
  aspect-ratio: 1/1;
  height: calc(var(--component-height)/6 - var(--small-clock-margin)*2 - var(--small-clock-border-width)*2);
  background-color: var(--small-clock-background-color);
  display: flex;
  justify-content: center;
  border-radius: 50%;
  margin: var(--small-clock-margin);
  border: var(--small-clock-border-width) solid var(--small-clock-border-color);
}
.arrow {
  width: var(--arrow-width);
  height: calc((var(--component-height)/6 - var(--small-clock-margin)*2 - var(--small-clock-border-width)*2) /2);
  background-color: var(--arrow-background-color);
  transition: var(--animation-time);
  position: absolute;
  transform-origin: bottom center;
}
</style>
