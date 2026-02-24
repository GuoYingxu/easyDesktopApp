/// <reference types="vite/client" />
declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}
 
interface Window {
  // 声明 __TAURI__ 为 Tauri 提供的全局对象类型
  __TAURI__: any
}