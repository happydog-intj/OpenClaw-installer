<template>
  <div class="install-options">
    <div class="container">
      <h2>选择安装方式</h2>
      <p class="subtitle">根据你的需求选择合适的安装方式</p>
      
      <div class="options">
        <div 
          class="option"
          :class="{ selected: selectedMethod === 'npm' }"
          @click="selectedMethod = 'npm'"
        >
          <div class="icon">📦</div>
          <h3>推荐：npm 全局安装</h3>
          <ul>
            <li>✓ 快速安装（约 2 分钟）</li>
            <li>✓ 自动更新</li>
            <li>✓ 适合日常使用</li>
          </ul>
        </div>
        
        <div 
          class="option"
          :class="{ selected: selectedMethod === 'git' }"
          @click="selectedMethod = 'git'"
        >
          <div class="icon">🔧</div>
          <h3>开发者：源码安装</h3>
          <ul>
            <li>✓ 最新功能</li>
            <li>✓ 可自定义修改</li>
            <li>✓ 适合开发调试</li>
          </ul>
        </div>
      </div>
      
      <div class="actions">
        <button @click="$emit('back')" class="btn-secondary">
          ← 返回
        </button>
        <button @click="startInstall" class="btn-primary">
          开始安装 →
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const emit = defineEmits(['start', 'back'])

const selectedMethod = ref('npm')

function startInstall() {
  emit('start', { method: selectedMethod.value })
}
</script>

<style scoped>
.install-options {
  width: 100%;
  max-width: 700px;
  padding: 40px;
}

.container {
  background: white;
  border-radius: 20px;
  padding: 50px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

h2 {
  font-size: 28px;
  font-weight: 700;
  color: #333;
  margin-bottom: 10px;
}

.subtitle {
  font-size: 16px;
  color: #666;
  margin-bottom: 30px;
}

.options {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
  margin-bottom: 30px;
}

.option {
  padding: 30px;
  border: 3px solid #e5e7eb;
  border-radius: 15px;
  cursor: pointer;
  transition: all 0.3s;
  text-align: center;
}

.option:hover {
  border-color: #667eea;
  transform: translateY(-5px);
}

.option.selected {
  border-color: #667eea;
  background: linear-gradient(135deg, #667eea10 0%, #764ba210 100%);
}

.option .icon {
  font-size: 50px;
  margin-bottom: 15px;
}

.option h3 {
  font-size: 18px;
  font-weight: 600;
  color: #333;
  margin-bottom: 15px;
}

.option ul {
  list-style: none;
  padding: 0;
  text-align: left;
}

.option li {
  font-size: 14px;
  color: #666;
  margin: 8px 0;
}

.actions {
  display: flex;
  gap: 15px;
}

.btn-secondary {
  background: #e5e7eb;
  color: #374151;
  border: none;
  padding: 12px 30px;
  font-size: 16px;
  font-weight: 600;
  border-radius: 10px;
  cursor: pointer;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 12px 30px;
  font-size: 16px;
  font-weight: 600;
  border-radius: 10px;
  cursor: pointer;
  flex: 1;
}
</style>
