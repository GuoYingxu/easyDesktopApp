import { createRouter, createWebHistory } from "vue-router";
import Detection from "../modules/detection/Detection.vue";
import Config from "../modules/detecConfig/Config.vue";
import History from "../modules/statistics/History.vue";
import Production from "../modules/production/Production.vue";
import Settings from "../modules/settings/Settings.vue";
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
		component:  Detection
	},
	{
		path: "/statistics",
		name: "Statistics",
    meta: { title: '数据统计' },
		component: History,
	},
	{
		path: "/config",
		name: "Config",
    meta: { title: '检测配置' },
		component:  Config
	},
	{
		path: "/history",
		name: "History",
    meta: { title: '历史数据' },
		component:History
	},
	{
		path: "/alerts",
		name: "Alerts",
    meta: { title: '产品信息' },
		component: Production
	},
	{
		path: "/settings",
		name: "Settings",
    meta: { title: '系统设置' },
		component:  Settings
	},
];

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes,
});

export default router;
