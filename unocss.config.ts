import { defineConfig, presetUno, presetAttributify, presetIcons } from 'unocss'

export default defineConfig({
  presets: [
    presetUno(),
    presetAttributify(),
    presetIcons({ scale: 1.2 }),
  ],
  rules: [
    // custom rule example: sidebar width helper
    ['w-sidebar', { width: '100px' }],
  ],
})
