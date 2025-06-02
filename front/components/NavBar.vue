<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

const props = defineProps({
  conditions: {
    type: Map,
    default: () => new Map(),
  },
  selectedFeatures: {
    type: Map,
    default: () => new Map(),
  }
});

const emits = defineEmits<{
  (e: "emit-bits", bits: [number, number]): void;
  (e: "feature-toggle", feature: any, isAdding: boolean): void;
  (e: "clear-filters"): void;
}>();

const activeMenu = ref<string | null>(null);

const menuCategories = computed(() => {
  if (!props.conditions || props.conditions.size === 0) return [];
  
  const keys = Array.from(props.conditions.keys()) as Array<string>;
  keys.sort((a, b) => a.localeCompare(b));
  
  return keys.map((key) => ({
    key: key,
    display: key.substring(2), // 先頭2文字を削除
    features: props.conditions.get(key) || []
  }));
});

const toggleMenu = (menuKey: string) => {
  activeMenu.value = activeMenu.value === menuKey ? null : menuKey;
};

const selectFeature = (feature: any) => {
  const featureId = `${feature.name}_${feature.bit_shift[0]}_${feature.bit_shift[1]}`;
  const wasSelected = props.selectedFeatures.has(featureId);
  
  console.log(`NavBar.selectFeature: ${feature.name}`);
  console.log(`Was selected: ${wasSelected}`);
  console.log(`Feature ID: ${featureId}`);
  
  if (wasSelected) {
    console.log(`Emitting feature-toggle with false (unchecking)`);
    emits('feature-toggle', feature, false); // 選択解除
  } else {
    console.log(`Emitting feature-toggle with true (checking)`);
    emits('feature-toggle', feature, true); // 選択追加
  }
  
  // 後方互換性のため、従来のイベントも送信
  emits('emit-bits', feature.bit_shift);
  // activeMenu.value = null; // メニューを開いたままにする
};

const isSelected = (feature: any) => {
  const featureId = `${feature.name}_${feature.bit_shift[0]}_${feature.bit_shift[1]}`;
  return props.selectedFeatures.has(featureId);
};

const hasSelectedChildren = (category: any) => {
  return category.features.some((feature: any) => isSelected(feature));
};

const closeMenu = (event: Event) => {
  // クリックがナビゲーション外の場合のみメニューを閉じる
  const target = event.target as Element;
  if (!target.closest('.nav-bar')) {
    activeMenu.value = null;
  }
};

const clearSelectedFeatures = () => {
  emits('clear-filters');
};

onMounted(() => {
  document.addEventListener('click', closeMenu);
});

onUnmounted(() => {
  document.removeEventListener('click', closeMenu);
});

defineExpose({
  clearSelectedFeatures
});
</script>

<template lang="pug">
  .nav-bar
    .nav-section.nav-links
      NuxtLink.nav-link(to="/card" :class="{ active: $route.path === '/card' }")
        span Cards
      NuxtLink.nav-link(to="/products" :class="{ active: $route.path === '/products' }")
        span Products
      NuxtLink.nav-link(to="/admin" :class="{ active: $route.path === '/admin' }")
        span Admin
    .nav-section.feature-menu(v-if="menuCategories.length > 0")
      .menu-item(
        v-for="category in menuCategories" 
        :key="category.key" 
        @click.stop="toggleMenu(category.key)" 
        :class="{ active: activeMenu === category.key, 'has-selected': hasSelectedChildren(category) }"
      )
        span.menu-label {{ category.display }}
        span.indicator(v-if="hasSelectedChildren(category)") ●
        .dropdown(v-if="activeMenu === category.key")
          .dropdown-item(
            v-for="feature in category.features" 
            :key="feature.name"
            @click.stop="selectFeature(feature)"
            :class="{ selected: isSelected(feature) }"
          )
            .checkbox
              .checkmark(v-if="isSelected(feature)") ✓
            span.feature-name {{ feature.name }}
</template>

<style scoped lang="less">
.nav-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: #2c3e50;
  color: white;
  padding: 0 20px;
  height: 50px;
  margin-bottom: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  position: relative;
  z-index: 1000;
}

.nav-section {
  display: flex;
  align-items: center;
}

.nav-links {
  gap: 0;
}

.nav-link {
  padding: 12px 20px;
  text-decoration: none;
  color: #bdc3c7;
  transition: all 0.2s ease;
  border-radius: 0;
  position: relative;
  
  &:hover {
    background-color: #34495e;
    color: white;
  }
  
  &.active {
    background-color: #3498db;
    color: white;
    
    &::after {
      content: '';
      position: absolute;
      bottom: 0;
      left: 0;
      right: 0;
      height: 3px;
      background-color: #e74c3c;
    }
  }
  
  span {
    font-size: 14px;
    font-weight: 500;
  }
}

.feature-menu {
  gap: 0;
}

.menu-item {
  position: relative;
  padding: 12px 16px;
  cursor: pointer;
  transition: background-color 0.2s ease;
  color: #bdc3c7;
  display: flex;
  align-items: center;
  
  &:hover {
    background-color: #34495e;
    color: white;
  }
  
  &.active {
    background-color: #3498db;
    color: white;
  }
  
  &.has-selected {
    background-color: #27ae60;
    color: white;
    
    &:hover {
      background-color: #2ecc71;
    }
    
    &.active {
      background-color: #3498db;
    }
  }
}

.menu-label {
  font-weight: 500;
  font-size: 14px;
  flex: 1;
}

.indicator {
  font-size: 8px;
  margin-left: 6px;
  color: #e74c3c;
  animation: pulse 1.5s ease-in-out infinite;
  
  .menu-item.has-selected & {
    color: #fff;
  }
}

@keyframes pulse {
  0% { opacity: 1; }
  50% { opacity: 0.6; }
  100% { opacity: 1; }
}

.dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  min-width: 200px;
  max-width: 300px;
  background-color: white;
  border: 1px solid #dee2e6;
  border-top: none;
  border-radius: 0 0 4px 4px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  z-index: 1001;
}

.dropdown-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  cursor: pointer;
  transition: background-color 0.2s ease;
  border-bottom: 1px solid #f8f9fa;
  color: #495057;
  
  &:last-child {
    border-bottom: none;
  }
  
  &:hover {
    background-color: #f8f9fa;
  }
  
  &.selected {
    background-color: #e3f2fd;
    color: #1976d2;
  }
}

.checkbox {
  width: 16px;
  height: 16px;
  border: 1px solid #ccc;
  border-radius: 2px;
  margin-right: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: white;
  transition: all 0.2s ease;
  
  .dropdown-item.selected & {
    border-color: #1976d2;
    background-color: #1976d2;
    color: white;
  }
}

.checkmark {
  font-size: 10px;
  font-weight: bold;
}

.feature-name {
  font-size: 13px;
  flex: 1;
}
</style>