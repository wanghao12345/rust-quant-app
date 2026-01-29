import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/login",
      component: () => import("@/views/auth/login.vue"),
    },
    {
      path: "/",
      component: () => import("@/components/Layout.vue"),
      redirect: "/dashboard",
      children: [
        {
          path: "dashboard",
          component: () => import("@/views/dashboard/index.vue"),
        },
        {
          path: "risk",
          component: () => import("@/views/risk/index.vue"),
        },
        {
          path: "strategy",
          component: () => import("@/views/strategy/index.vue"),
        },
        {
          path: "user",
          component: () => import("@/views/user/index.vue"),
        },
      ],
    },
  ],
});

export default router;