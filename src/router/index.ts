import { createRouter, createWebHistory } from "vue-router";

const routes = [
	// redirect root to real-time detection
	{
		path: "/",
		redirect: "/detection",
	},
	{
		path: "/detection",
		name: "Detection",
    meta: { title: '实时外观检测' },
		component: () => import("../modules/detection/Detection.vue"),
	},
	{
		path: "/statistics",
		name: "Statistics",
    meta: { title: '数据统计' },
		component: () => import("../views/Statistics.vue"),
	},
	{
		path: "/config",
		name: "Config",
    meta: { title: '检测配置' },
		component: () => import("../views/Config.vue"),
	},
	{
		path: "/history",
		name: "History",
    meta: { title: '历史数据' },
		component: () => import("../views/History.vue"),
	},
	{
		path: "/alerts",
		name: "Alerts",
    meta: { title: '报警管理' },
		component: () => import("../views/Alerts.vue"),
	},
	{
		path: "/settings",
		name: "Settings",
    meta: { title: '系统设置' },
		component: () => import("../views/Settings.vue"),
	},
];

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes,
});

export default router;
