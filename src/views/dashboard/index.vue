<template>
  <div>
    仪表盘
    <button @click="fetchKlines">获取K线数据</button>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';

const fetchKlines = async () => {
  const request = {
    symbol: 'BTCUSDT',
    interval: '1m',
    limit: 100,
  };
  await invoke('fetch_klines', { request });
};

</script>
