export default defineNuxtConfig({
  ssr: false,
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  modules: ['@nuxtjs/tailwindcss'],
  nitro: {
    output: {
      publicDir: '.output/public'
    }
  },
  app: {
    baseURL: './',
    buildAssetsDir: 'assets'
  }
})