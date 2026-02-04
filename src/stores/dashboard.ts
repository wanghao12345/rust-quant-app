import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

// 定义类型
interface AssetOverview {
  totalAsset: number;
  totalChange: number;
  totalChangePercent: number;
  dayPnL: number;
  dayPnLPercent: number;
  positionCount: number;
  activeStrategies: number;
}

interface MarketData {
  symbol: string;
  name: string;
  price: number;
  change24h: number;
  change24hPercent: number;
  volume24h: number;
  marketCap: number;
}

interface KlineData {
  time: number;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

export const useDashboardStore = defineStore('dashboard', () => {
  // 状态
  const assetOverview = ref<AssetOverview>({
    totalAsset: 105680.50,
    totalChange: 2350.75,
    totalChangePercent: 2.28,
    dayPnL: 850.25,
    dayPnLPercent: 0.81,
    positionCount: 12,
    activeStrategies: 8
  });

  const marketData = ref<MarketData[]>([
    {
      symbol: 'BTC',
      name: 'Bitcoin',
      price: 42500.00,
      change24h: 1250.00,
      change24hPercent: 3.03,
      volume24h: 15200000000,
      marketCap: 825000000000
    },
    {
      symbol: 'ETH',
      name: 'Ethereum',
      price: 2200.00,
      change24h: 45.50,
      change24hPercent: 2.11,
      volume24h: 8500000000,
      marketCap: 265000000000
    },
    {
      symbol: 'SOL',
      name: 'Solana',
      price: 115.00,
      change24h: 3.25,
      change24hPercent: 2.91,
      volume24h: 1200000000,
      marketCap: 45000000000
    },
    {
      symbol: 'ADA',
      name: 'Cardano',
      price: 0.55,
      change24h: 0.015,
      change24hPercent: 2.81,
      volume24h: 350000000,
      marketCap: 19000000000
    },
    {
      symbol: 'DOT',
      name: 'Polkadot',
      price: 7.80,
      change24h: 0.15,
      change24hPercent: 1.96,
      volume24h: 280000000,
      marketCap: 8500000000
    }
  ]);

  const selectedSymbol = ref('BTC');
  const timeRange = ref('1D');

  // 生成模拟 K 线数据
  const generateKlineData = (symbol: string, range: string): KlineData[] => {
    const data: KlineData[] = [];
    const now = Date.now();
    let price = symbol === 'BTC' ? 41000 : symbol === 'ETH' ? 2100 : 110;
    const interval = range === '1H' ? 3600000 : range === '4H' ? 14400000 : range === '1D' ? 86400000 : 604800000;
    const count = range === '1H' ? 24 : range === '4H' ? 24 : range === '1D' ? 30 : 12;

    for (let i = count; i >= 0; i--) {
      const time = now - i * interval;
      const open = price;
      const high = price * (1 + Math.random() * 0.02);
      const low = price * (1 - Math.random() * 0.02);
      const close = low + (high - low) * Math.random();
      const volume = Math.random() * 1000000000;

      data.push({
        time,
        open,
        high,
        low,
        close,
        volume
      });

      price = close;
    }

    return data;
  };

  const klineData = computed(() => {
    return generateKlineData(selectedSymbol.value, timeRange.value);
  });

  // 操作
  const updateAssetOverview = () => {
    // 模拟数据更新
    assetOverview.value = {
      ...assetOverview.value,
      totalAsset: 105680.50 + Math.random() * 500 - 250,
      dayPnL: 850.25 + Math.random() * 100 - 50,
      dayPnLPercent: 0.81 + (Math.random() * 0.2 - 0.1)
    };
  };

  const updateMarketData = () => {
    // 模拟市场数据更新
    marketData.value = marketData.value.map(item => ({
      ...item,
      price: item.price * (1 + (Math.random() * 0.02 - 0.01)),
      change24h: item.change24h * (1 + (Math.random() * 0.2 - 0.1)),
      change24hPercent: item.change24hPercent * (1 + (Math.random() * 0.2 - 0.1)),
      volume24h: item.volume24h * (1 + (Math.random() * 0.3 - 0.15))
    }));
  };

  const setSelectedSymbol = (symbol: string) => {
    selectedSymbol.value = symbol;
  };

  const setTimeRange = (range: string) => {
    timeRange.value = range;
  };

  const refreshAllData = () => {
    updateAssetOverview();
    updateMarketData();
  };

  // 快速操作
  const closeAllPositions = () => {
    // 模拟平仓操作
    console.log('Closing all positions...');
    assetOverview.value.positionCount = 0;
    return true;
  };

  const pauseAllStrategies = () => {
    // 模拟暂停所有策略
    console.log('Pausing all strategies...');
    assetOverview.value.activeStrategies = 0;
    return true;
  };

  return {
    // 状态
    assetOverview,
    marketData,
    selectedSymbol,
    timeRange,
    klineData,
    
    // 计算属性
    
    // 操作
    updateAssetOverview,
    updateMarketData,
    setSelectedSymbol,
    setTimeRange,
    refreshAllData,
    closeAllPositions,
    pauseAllStrategies
  };
});
