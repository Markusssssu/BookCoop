// nuxt.config.ts
export default defineNuxtConfig({
  modules: [
    '@nuxt/eslint',
    '@nuxt/ui'
  ],

  css: ['~/assets/css/main.css'],

  devtools: { enabled: true },

  routeRules: {
    '/': { prerender: true },
    '/books/**': { ssr: true },
    '/authors/**': { ssr: true }
  },

  compatibilityDate: '2025-01-15'
})
