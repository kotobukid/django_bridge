export default defineNuxtConfig({
  ssr: false,
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  modules: ['@nuxtjs/tailwindcss'],
  nitro: {
    output: {
      publicDir: '.output/public'
    },
    devProxy: {
      '/api': {
        target: 'http://localhost:3030',
        changeOrigin: true
      }
    }
  },
  app: {
    baseURL: './',
    buildAssetsDir: 'assets'
  },
  vite: {
    server: {
      proxy: {
        '/api': {
          target: 'http://localhost:3030',
          changeOrigin: true
        }
      }
    }
  }
})