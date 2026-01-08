/**
 * Production line configuration for the host (上位机).
 * 包含产线名称和工位名称。
 */
export interface ProductionLineConfig {
  productionLineName: string;
  workstationName: string;
}
const defaultConfig: ProductionLineConfig = {
  productionLineName: '冰箱总装产线#1',
  workstationName: '工位：外观检测工位1',
};

/**
 * 运行时配置访问器
 * 提供获取和更新产线配置的方法
 */
let runtimeConfig: ProductionLineConfig = { ...defaultConfig };

export function getConfig(): ProductionLineConfig {
  return { ...runtimeConfig };
}

export function setConfig(update: Partial<ProductionLineConfig>): void {
  runtimeConfig = { ...runtimeConfig, ...update };
}

export function resetConfig(): void {
  runtimeConfig = { ...defaultConfig };
}

export default {
  getConfig,
  setConfig,
  resetConfig,
};
