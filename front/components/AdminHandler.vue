<script setup lang="ts">
import axios, {type AxiosResponse} from 'axios';
import {ref, computed, onUnmounted} from 'vue';

type DjangoStartResult = {
  success: boolean,
  entry?: string,
}

type DjangoStatusResult = {
  success: boolean,
  is_running: boolean,
  admin_root?: string,
  error?: string,
}

const admin_alive = ref(false);
const admin_entry = ref("");
const timer = ref<number | null>(null);
const is_polling = computed(() => timer.value !== null);

// ヘルスチェック実行回数（視認性のため）
const health_check_count = ref(0);

// ボタンのローディング状態
const starting_admin = ref(false);
const stopping_admin = ref(false);

const startAdmin = () => {
  starting_admin.value = true;
  axios.post("/admin_operation/api/start_admin.json").then((data: AxiosResponse<DjangoStartResult>) => {
    if (data.data.success) {
      start_health_check_polling();
      if (data.data.entry) {
        admin_entry.value = "/" + data.data.entry;
      }
    }
  }).catch((error) => {
    console.error("Failed to start admin:", error);
  }).finally(() => {
    starting_admin.value = false;
  });
}

const stopAdmin = () => {
  stopping_admin.value = true;
  axios.post("/admin_operation/api/stop_admin.json").then((data: AxiosResponse<DjangoStartResult>) => {
    if (data.data.success) {
      admin_entry.value = "";
      admin_alive.value = false; // サーバー停止状態を明示的に設定
      stop_health_check_polling(); // ポーリング停止を追加
      
      // 停止後、少し待ってから状態を確認（確実な状態反映のため）
      setTimeout(() => {
        health_check();
      }, 500);
    }
    console.log(data.data);
  }).catch((error) => {
    console.error("Failed to stop admin:", error);
    // エラーが発生してもヘルスチェックで状態を確認
    health_check();
  }).finally(() => {
    stopping_admin.value = false;
  });
}

const health_check = () => {
  health_check_count.value++;
  axios.get("/admin_proxy/health-check", {
    timeout: 1000,
  }).then((data: AxiosResponse<string>) => {
    admin_alive.value = (data.data === 'OK');
  }).catch((_err) => {
    admin_alive.value = false;
  });
}

const start_health_check_polling = () => {
  if (timer.value !== null) {
    clearInterval(timer.value);
  }
  timer.value = setInterval(health_check, 2000);
}

const stop_health_check_polling = () => {
  if (timer.value !== null) {
    clearInterval(timer.value);
    timer.value = null;
  }
}

const toggle_health_check = () => {
  if (is_polling.value) {
    stop_health_check_polling();
  } else {
    start_health_check_polling();
  }
}

const get_admin_status = async () => {
  try {
    const response = await axios.get("/admin_operation/api/status.json");
    const data: DjangoStatusResult = response.data;
    
    if (data.success) {
      admin_alive.value = data.is_running;
      
      if (data.is_running && data.admin_root) {
        // 管理画面ルートが取得できた場合
        admin_entry.value = "/" + data.admin_root;
      } else {
        // サーバーが停止している、またはルートが取得できない場合
        admin_entry.value = "";
      }
    } else {
      console.error("Failed to get admin status:", data.error);
      admin_alive.value = false;
      admin_entry.value = "";
    }
  } catch (error) {
    console.error("Error getting admin status:", error);
    admin_alive.value = false;
    admin_entry.value = "";
  }
}

// 初期化処理: サーバー状態の確認
const initialize = async () => {
  // まず状態APIで詳細情報を取得
  await get_admin_status();
  
  // その後、ヘルスチェックを一度実行
  health_check();
  
  // サーバーが起動している場合のみポーリングを開始
  if (admin_alive.value) {
    start_health_check_polling();
  }
}

// 初期化実行
initialize();

// コンポーネント破棄時のクリーンアップ
onUnmounted(() => {
  stop_health_check_polling();
});
</script>

<template lang="pug">
  .admin-handler(:class="{ 'polling-active': is_polling }")
    h1 Admin Handler
    
    // サーバー制御ボタン
    .server-controls
      v-btn(@click="startAdmin" v-if="!admin_alive" color="primary" :loading="starting_admin" :disabled="starting_admin" prepend-icon="mdi-play") 
        | {{ starting_admin ? 'Starting...' : 'Start Admin' }}
      template(v-if="admin_alive")
        v-btn(@click="stopAdmin" color="error" :loading="stopping_admin" :disabled="stopping_admin" prepend-icon="mdi-stop")
          | {{ stopping_admin ? 'Stopping...' : 'Stop Admin' }}
        v-btn(:href="admin_entry" target="_blank" color="secondary" variant="outlined" prepend-icon="mdi-open-in-new" v-if="admin_entry")
          | Open Admin Panel
    
    // ステータス表示
    .status-display
      .server-status
        v-chip(:color="admin_alive ? 'success' : 'error'" variant="flat" size="small" :prepend-icon="admin_alive ? 'mdi-check-circle' : 'mdi-close-circle'")
          | {{ admin_alive ? 'Server Running' : 'Server Stopped' }}
      
      .polling-status
        v-chip(
          :color="is_polling ? 'info' : 'default'" 
          variant="outlined" 
          size="small"
          @click="toggle_health_check"
          style="cursor: pointer"
          :prepend-icon="is_polling ? 'mdi-radar' : 'mdi-radar-off'"
        )
          | {{ is_polling ? 'Polling Active' : 'Polling Inactive' }}
          span(v-if="is_polling") &nbsp;({{ health_check_count }})
    
    // デバッグ情報（開発時用）
    .debug-info(v-if="false")
      small Timer ID: {{ timer }}
      br
      small Is Polling: {{ is_polling }}
      br  
      small Health Check Count: {{ health_check_count }}
</template>

<style scoped lang="less">
.admin-handler {
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 16px;
  width: 500px;
  background: #fafafa;
  
  h1 {
    margin-bottom: 16px;
    color: #333;
    font-size: 1.5rem;
  }
  
  .server-controls {
    margin-bottom: 16px;
    display: flex;
    gap: 12px;
    align-items: center;
    
    .v-btn {
      min-width: 140px;
    }
  }
  
  .status-display {
    display: flex;
    gap: 12px;
    margin-bottom: 16px;
    align-items: center;
    flex-wrap: wrap;
    
    .server-status, .polling-status {
      .v-chip {
        font-weight: 500;
      }
    }
    
    .polling-status .v-chip {
      transition: all 0.3s ease;
      
      &:hover {
        transform: scale(1.05);
      }
    }
  }
  
  .debug-info {
    margin-top: 16px;
    padding: 8px;
    background: #f5f5f5;
    border-radius: 4px;
    font-family: monospace;
    opacity: 0.7;
  }
}

// ポーリング中のアニメーション効果
.polling-active {
  .v-chip {
    animation: pulse 2s infinite;
  }
}

@keyframes pulse {
  0% { opacity: 1; }
  50% { opacity: 0.7; }
  100% { opacity: 1; }
}
</style>