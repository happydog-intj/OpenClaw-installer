<template>
  <div class="dependency-check">
    <div class="container">
      <h2>系统环境检测</h2>
      <p class="subtitle">正在检测必需组件...</p>
      
      <div v-if="loading" class="loading">
        <div class="spinner"></div>
        <p>检测中...</p>
      </div>
      
      <div v-else class="dependencies">
        <div 
          v-for="dep in dependencies" 
          :key="dep.name"
          class="dep-item"
          :class="getDepClass(dep)"
        >
          <div class="dep-icon">
            <span v-if="dep.installed">✅</span>
            <span v-else-if="!dep.required">⚪</span>
            <span v-else>⚠️</span>
          </div>
          
          <div class="dep-info">
            <h3>{{ dep.displayName }}</h3>
            <p v-if="dep.installed" class="version">
              已安装: {{ dep.currentVersion }}
              <span v-if="dep.needsUpdate" class="warning">(需要更新至 {{ dep.requiredVersion }})</span>
            </p>
            <p v-else class="missing">
              未安装 (需要 {{ dep.requiredVersion }})
            </p>
          </div>
          
          <button 
            v-if="(!dep.installed || dep.needsUpdate) && dep.installCommand"
            @click="installDep(dep.name)"
            :disabled="installing === dep.name"
            class="btn-install"
          >
            {{ installing === dep.name ? '安装中...' : '立即安装' }}
          </button>
        </div>
      </div>
      
      <div class="summary" v-if="!loading">
        <div v-if="allDependenciesMet" class="success">
          ✓ 所有依赖已满足，可以继续安装
        </div>
        <div v-else class="warning">
          ⚠️ 请先安装缺失的依赖项
        </div>
      </div>
      
      <div class="actions">
        <button @click="$emit('back')" class="btn-secondary">
          ← 返回
        </button>
        <button 
          @click="$emit('next')"
          :disabled="!allDependenciesMet"
          class="btn-primary"
        >
          继续 →
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

defineEmits(['next', 'back'])

interface Dependency {
  name: string
  displayName: string
  required: boolean
  requiredVersion: string
  currentVersion?: string
  installed: boolean
  needsUpdate: boolean
  installCommand?: string
}

const loading = ref(true)
const dependencies = ref<Dependency[]>([])
const installing = ref<string | null>(null)

const allDependenciesMet = computed(() => {
  return dependencies.value
    .filter(d => d.required)
    .every(d => d.installed && !d.needsUpdate)
})

onMounted(async () => {
  await checkDependencies()
})

async function checkDependencies() {
  loading.value = true
  try {
    dependencies.value = await invoke('check_system_dependencies') as Dependency[]
  } catch (error) {
    console.error('检测依赖失败:', error)
  } finally {
    loading.value = false
  }
}

async function installDep(name: string) {
  installing.value = name
  try {
    await invoke('install_dependency', { name })
    await checkDependencies() // 重新检测
  } catch (error) {
    console.error(`安装 ${name} 失败:`, error)
    alert(`安装失败: ${error}`)
  } finally {
    installing.value = null
  }
}

function getDepClass(dep: Dependency) {
  if (dep.installed && !dep.needsUpdate) return 'success'
  if (dep.required) return 'error'
  return 'optional'
}
</script>

<style scoped>
.dependency-check {
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

.loading {
  text-align: center;
  padding: 60px 0;
}

.spinner {
  width: 50px;
  height: 50px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.dependencies {
  margin-bottom: 30px;
}

.dep-item {
  display: flex;
  align-items: center;
  padding: 20px;
  margin-bottom: 15px;
  border-radius: 10px;
  transition: all 0.3s;
}

.dep-item.success {
  background: #f0fdf4;
  border: 2px solid #10b981;
}

.dep-item.error {
  background: #fef2f2;
  border: 2px solid #ef4444;
}

.dep-item.optional {
  background: #f9fafb;
  border: 2px solid #d1d5db;
}

.dep-icon {
  font-size: 32px;
  margin-right: 20px;
}

.dep-info {
  flex: 1;
}

.dep-info h3 {
  font-size: 18px;
  font-weight: 600;
  color: #333;
  margin-bottom: 5px;
}

.dep-info .version {
  font-size: 14px;
  color: #10b981;
}

.dep-info .missing {
  font-size: 14px;
  color: #ef4444;
}

.dep-info .warning {
  color: #f59e0b;
  margin-left: 10px;
}

.btn-install {
  background: #667eea;
  color: white;
  border: none;
  padding: 10px 20px;
  font-size: 14px;
  font-weight: 600;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-install:hover:not(:disabled) {
  background: #5568d3;
}

.btn-install:disabled {
  background: #cbd5e1;
  cursor: not-allowed;
}

.summary {
  padding: 20px;
  border-radius: 10px;
  margin-bottom: 30px;
  text-align: center;
  font-weight: 600;
}

.summary.success {
  background: #f0fdf4;
  color: #10b981;
}

.summary.warning {
  background: #fef2f2;
  color: #ef4444;
}

.actions {
  display: flex;
  gap: 15px;
  justify-content: space-between;
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
  transition: background 0.2s;
}

.btn-secondary:hover {
  background: #d1d5db;
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
  transition: transform 0.2s, box-shadow 0.2s;
  flex: 1;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 10px 20px rgba(102, 126, 234, 0.4);
}

.btn-primary:disabled {
  background: #cbd5e1;
  cursor: not-allowed;
  transform: none;
}
</style>
