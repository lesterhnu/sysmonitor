<script setup lang="ts">
import { ref } from 'vue';
import { AppleOutlined, AndroidOutlined } from '@ant-design/icons-vue';
const activeKey = ref(1);
</script>

<template>
  <a-tabs v-model:activeKey="activeKey">
    <a-tab-pane key="1">
      <template #tab>
        <span>
          <apple-outlined />
          Tab 1
        </span>
      </template>
      Tab 1
    </a-tab-pane>
    <a-tab-pane key="2">
      <template #tab>
        <span>
          <android-outlined />
          Tab 2
        </span>
      </template>
      Tab 2
    </a-tab-pane>
  </a-tabs>
</template>

<style scoped>


</style>