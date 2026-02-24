# 项目架构规则

## 技术栈
- 前端：Vue 3 + TypeScript
- 后端：Rust + Tauri 2
- UI框架：Element Plus
- CSS框架：UnoCSS
- 状态管理：Pinia

## CSS 规范
- 使用 UnoCSS 作为主要的 CSS 框架
- 避免使用传统的 CSS 类名，优先使用 UnoCSS 的原子类
- 遵循 UnoCSS 的实用优先原则
- 在模板中直接使用 UnoCSS 的原子类名

## 其他规范
- 组件拆分：每个功能模块应拆分为独立的组件
- 状态管理：使用 Pinia 进行全局状态管理
- API 调用：通过 Tauri 命令与后端 Rust 代码通信
- 数据模型：在 models 目录中定义接口类型
- 存储：使用 stores 目录管理 Pinia store