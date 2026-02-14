<template>
  <div class="app">
    <WelcomeScreen 
      v-if="currentStep === 'welcome'" 
      @next="currentStep = 'check'" 
    />
    
    <DependencyCheck 
      v-else-if="currentStep === 'check'"
      @next="currentStep = 'options'"
      @back="currentStep = 'welcome'"
    />
    
    <InstallOptions
      v-else-if="currentStep === 'options'"
      @start="startInstallation"
      @back="currentStep = 'check'"
    />
    
    <InstallProgress
      v-else-if="currentStep === 'progress'"
      :progress="installProgress"
    />
    
    <SuccessScreen
      v-else-if="currentStep === 'success'"
      @close="closeApp"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import WelcomeScreen from './components/WelcomeScreen.vue'
import DependencyCheck from './components/DependencyCheck.vue'
import InstallOptions from './components/InstallOptions.vue'
import InstallProgress from './components/InstallProgress.vue'
import SuccessScreen from './components/SuccessScreen.vue'

type Step = 'welcome' | 'check' | 'options' | 'progress' | 'success'

const currentStep = ref<Step>('welcome')
const installProgress = ref({
  step: '',
  status: '',
  progress: 0,
  message: '',
  logs: [] as string[]
})

// 监听安装进度事件
listen('install-progress', (event: any) => {
  installProgress.value = event.payload
  
  // 安装成功后切换到成功页面
  if (event.payload.status === 'success' && event.payload.progress === 100) {
    setTimeout(() => {
      currentStep.value = 'success'
    }, 1000)
  }
})

async function startInstallation(options: { method: string, customPath?: string }) {
  currentStep.value = 'progress'
  
  try {
    await invoke('start_installation', { options })
  } catch (error) {
    console.error('安装失败:', error)
    // TODO: 显示错误页面
  }
}

function closeApp() {
  window.close()
}
</script>

<style scoped>
.app {
  width: 100%;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}
</style>
