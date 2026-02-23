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
      
      <!-- OpenClaw 已安装提示 -->
      <div v-if="openclawInstalled && !loading" class="openclaw-installed-notice">
        <div class="notice-header">
          <span class="notice-icon">🎉</span>
          <h3>检测到已安装 OpenClaw</h3>
        </div>
        <p class="version-info">当前版本: {{ openclawVersion }}</p>
        
        <div class="install-options">
          <div 
            class="option-card"
            :class="{ selected: skipInstall }"
            @click="skipInstall = true"
          >
            <div class="option-icon">⏭️</div>
            <h4>跳过安装</h4>
            <p>直接进入配置向导</p>
          </div>
          
          <div 
            class="option-card"
            :class="{ selected: !skipInstall }"
            @click="skipInstall = false"
          >
            <div class="option-icon">⬆️</div>
            <h4>升级到最新版</h4>
            <p>重新安装并更新</p>
          </div>
        </div>
      </div>
      
      <div class="summary" v-if="!loading && !openclawInstalled">
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
          v-if="openclawInstalled && skipInstall"
          @click="handleSkipToConfig"
          :disabled="!allBasicDependenciesMet"
          class="btn-primary"
        >
          跳过安装，进入配置 →
        </button>
        <button 
          v-else
          @click="handleNext"
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

const emits = defineEmits(['next', 'back', 'skip-to-config'])

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

const openclawInstalled = computed(() => {
  const openclaw = dependencies.value.find(d => d.name === 'openclaw')
  return openclaw?.installed || false
})

const openclawVersion = computed(() => {
  const openclaw = dependencies.value.find(d => d.name === 'openclaw')
  return openclaw?.currentVersion || null
})

const allDependenciesMet = computed(() => {
  return dependencies.value
    .filter(d => d.required)
    .every(d => d.installed && !d.needsUpdate)
})

// 基础依赖检查（不包括 openclaw，只检查 Node.js, npm 等）
const allBasicDependenciesMet = computed(() => {
  return dependencies.value
    .filter(d => d.required && d.name !== 'openclaw')
    .every(d => d.installed && !d.needsUpdate)
})

const skipInstall = ref(false)

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

function handleNext() {
  if (!allDependenciesMet.value) {
    alert('请先安装所有必需的依赖项')
    return
  }
  emits('next')
}

function handleSkipToConfig() {
  if (!allBasicDependenciesMet.value) {
    alert('基础依赖未满足，无法进入配置。\n请先安装 Node.js 和 npm。')
    return
  }
  emits('skip-to-config')
}
</script>

<style scoped>
.dependency-check {
  width: 100%;
  max-width: 800px;
  padding: 20px;
  height: 100vh;
  display: flex;
  align-items: center;
}

.container {
  background: white;
  border-radius: 20px;
  padding: 30px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  width: 100%;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

h2 {
  font-size: 17px;
  font-weight: 700;
  color: #333;
  margin-bottom: 6px;
}

.subtitle {
  font-size: 10px;
  color: #666;
  margin-bottom: 15px;
}

.loading {
  text-align: center;
  padding: 30px 0;
  font-size: 10px;
}

.spinner {
  width: 30px;
  height: 30px;
  border: 3px solid #f3f3f3;
  border-top: 3px solid #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 12px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.dependencies {
  flex: 1;
  overflow-y: auto;
  margin-bottom: 20px;
  padding-right: 5px;
  scrollbar-width: thin;
  scrollbar-color: #cbd5e1 #f3f4f6;
}

.dependencies::-webkit-scrollbar {
  width: 6px;
}

.dependencies::-webkit-scrollbar-track {
  background: #f3f4f6;
  border-radius: 10px;
}

.dependencies::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 10px;
}

.dependencies::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}

.dep-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  margin-bottom: 6px;
  border-radius: 6px;
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
  font-size: 17px;
  margin-right: 10px;
}

.dep-info {
  flex: 1;
}

.dep-info h3 {
  font-size: 11px;
  font-weight: 600;
  color: #333;
  margin-bottom: 2px;
}

.dep-info .version {
  font-size: 8px;
  color: #10b981;
}

.dep-info .missing {
  font-size: 8px;
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
  padding: 6px 12px;
  font-size: 8px;
  font-weight: 600;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
  white-space: nowrap;
}

.btn-install:hover:not(:disabled) {
  background: #5568d3;
}

.btn-install:disabled {
  background: #cbd5e1;
  cursor: not-allowed;
}

.summary {
  padding: 10px 12px;
  border-radius: 6px;
  margin-bottom: 12px;
  text-align: center;
  font-weight: 600;
  font-size: 10px;
}

.summary.success {
  background: #f0fdf4;
  color: #10b981;
}

.summary.warning {
  background: #fef2f2;
  color: #ef4444;
}

.openclaw-installed-notice {
  background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
  border: 2px solid #3b82f6;
  border-radius: 10px;
  padding: 12px;
  margin-bottom: 12px;
}

.notice-header {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 4px;
}

.notice-icon {
  font-size: 17px;
}

.notice-header h3 {
  font-size: 11px;
  font-weight: 700;
  color: #1e40af;
  margin: 0;
}

.version-info {
  font-size: 9px;
  color: #1e40af;
  margin-bottom: 10px;
  font-weight: 500;
}

.install-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

.option-card {
  background: white;
  border: 2px solid #bfdbfe;
  border-radius: 8px;
  padding: 12px;
  cursor: pointer;
  transition: all 0.3s;
  text-align: center;
}

.option-card:hover {
  border-color: #3b82f6;
  transform: translateY(-2px);
  box-shadow: 0 6px 15px rgba(59, 130, 246, 0.2);
}

.option-card.selected {
  border-color: #3b82f6;
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.1) 0%, rgba(147, 197, 253, 0.1) 100%);
  box-shadow: 0 3px 12px rgba(59, 130, 246, 0.3);
}

.option-icon {
  font-size: 22px;
  margin-bottom: 6px;
}

.option-card h4 {
  font-size: 10px;
  font-weight: 700;
  color: #1e40af;
  margin-bottom: 3px;
}

.option-card p {
  font-size: 8px;
  color: #64748b;
  margin: 0;
}

.actions {
  display: flex;
  gap: 8px;
  justify-content: space-between;
  padding-top: 12px;
  border-top: 2px solid #e5e7eb;
}

.btn-secondary {
  background: #e5e7eb;
  color: #374151;
  border: none;
  padding: 8px 16px;
  font-size: 10px;
  font-weight: 600;
  border-radius: 6px;
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
  padding: 8px 16px;
  font-size: 10px;
  font-weight: 600;
  border-radius: 6px;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
  flex: 1;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 15px rgba(102, 126, 234, 0.4);
}

.btn-primary:disabled {
  background: #cbd5e1;
  cursor: not-allowed;
  transform: none;
}
</style>
