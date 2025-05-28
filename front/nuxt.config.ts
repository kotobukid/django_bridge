// https://nuxt.com/docs/api/configuration/nuxt-config
import vuetify, {transformAssetUrls} from 'vite-plugin-vuetify';
const webPort = process.env.WEB_PORT || '80';
const backend_origin = `http://127.0.0.1:${webPort}`;

const site_name = 'Card Manager';

export default defineNuxtConfig({
  // router: {
  //     options: {
  //        history: createWebHashHistory()
  //     }
  // },
  css: [
    '@mdi/font/css/materialdesignicons.css', // MDIフォントCSSを追加
    'vuetify/styles', // Vuetifyのスタイルを追加
  ],
  build: {
    transpile: ['vuetify'],
    // publicPath: "/static"
  },
  modules: [
    (_options, nuxt) => {
      nuxt.hooks.hook('vite:extendConfig', (config) => {
        // @ts-expect-error
        config.plugins.push(vuetify({autoImport: true}));
      });
    },
    "@pinia/nuxt",
  ],
  ssr: false, // SPA専用
  app: {
    head: {
      title: site_name,
      link: [
        {rel: "icon", type: "image/x-icon", href: "/favicon.ico"},
        // 必要に応じて他のアイコンも追加
        // { rel: "icon", type: "image/png", href: "/favicon-32x32.png", sizes: "32x32" },
        // { rel: "icon", type: "image/svg+xml", href: "/favicon.svg" }
      ]
    },
  },
  runtimeConfig: {
    public: {
      siteName: site_name,
    },
  },
  compatibilityDate: '2024-11-01',
  devtools: {enabled: true},
  vite: {
    server: {
      proxy: {
        '/admin_proxy': {
          target: backend_origin,
          changeOrigin: true,
        },
        '/a_static': {
          target: backend_origin,
          changeOrigin: true,
        },
        '/admin_operation/api': {
          target: backend_origin,
          changeOrigin: true,
        },
        '/api': {
          target: backend_origin,
          changeOrigin: true,
        },
      },
    },
    vue: {
      template: {
        transformAssetUrls,
      },
    },
  },
  devServer: {
    port: 3001,
  }
});