<template>
  <div class="flex h-screen bg-background">
    <!-- 左侧菜单栏 -->
    <div class="hidden md:flex flex-col w-56 bg-surface shadow-lg z-10">
      <!-- Logo -->
      <div class="p-6 border-b border-gray-200">
        <div class="flex items-center">
          <div class="w-10 h-10 box-border border-1 border-solid border-secondary rounded-md bg-secondary overflow-hidden flex items-center justify-center mr-2">
            <!-- <img src="/logo.png" alt="点赞AI" class="w-9 h-9 object-cover"> -->
          </div>
          <div>
            <h1 class="text-sm font-bold my-0">量化</h1>
            <p class="text-xs mt-1 mb-0">让每个策略都有价值！</p>
          </div>
        </div>
      </div>
      
      <!-- 菜单列表 -->
      <nav class="flex-1 p-4">
        <div 
          v-for="item in state.menus" 
          :key="item.path"
          @click="handleMenuClick(item.path)"
          :class="{'text-primary bg-primary/10': item.path === $route.path}" 
          class="w-full mb-2 flex items-center cursor-pointer box-border px-4 py-3 rounded-8px  hover:bg-gray-100">
          <i :class="`iconfont ${item.icon} text-22px`"></i>
          <span class="ml-2 text-15px">{{ item.title }}</span>
        </div>
      </nav>

    </div>

    <!-- 右侧内容区域 -->
    <div class="flex-1 overflow-y-auto p-4 md:p-8">
      <router-view />
    </div>
  </div>
</template>

<script setup>
const router = useRouter();
const state = reactive({
  menus: [
    {
      title: "仪表盘",
      icon: "icon-ziliao",
      path: "/dashboard"
    },
    {
      title: "策略中心",
      icon: "icon-huati",
      path: "/strategy"
    },
    {
      title: "风控中心",
      icon: "icon-huati",
      path: "/risk"
    },
    {
      title: "个人中心",
      icon: "icon-gerenzhongxin-zhihui",
      path: "/user"
    },
  ]
});

function handleMenuClick(path) {
  router.push(path);
}
</script>

<style scoped>
/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: #c4b5fd;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a78bfa;
}

/* 激活状态样式 */

</style>