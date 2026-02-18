<template>
  <div class="install-progress">
    <div class="container">
      <h2>正在安装 OpenClaw</h2>
      
      <div class="progress-section">
        <div class="progress-bar">
          <div 
            class="progress-fill"
            :style="{ width: `${progress.progress}%` }"
          ></div>
        </div>
        <p class="progress-text">{{ Math.round(progress.progress) }}%</p>
      </div>
      
      <div class="current-step">
        <h3>{{ progress.step || '准备中...' }}</h3>
      </div>
      
      <div class="logs">
        <h4>安装日志</h4>
        <div class="log-content" ref="logContainer">
          <p v-for="(log, i) in progress.logs" :key="i">{{ log }}</p>
          <p v-if="!progress.logs || progress.logs.length === 0" class="log-empty">
            等待安装开始...
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'

const props = defineProps<{
  progress: {
    step: string
    status: string
    progress: number
    message: string
    logs: string[]
  }
}>()

const logContainer = ref<HTMLElement | null>(null)

// 自动滚动到日志底部
watch(() => props.progress.logs, async () => {
  await nextTick()
  if (logContainer.value) {
    logContainer.value.scrollTop = logContainer.value.scrollHeight
  }
}, { deep: true })
</script>

<style scoped>
.install-progress {
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
  height: 90vh;
  max-height: 700px;
  display: flex;
  flex-direction: column;
}

h2 {
  font-size: 24px;
  font-weight: 700;
  color: #333;
  margin-bottom: 20px;
  text-align: center;
}

.progress-section {
  margin-bottom: 20px;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: #e5e7eb;
  border-radius: 10px;
  overflow: hidden;
  margin-bottom: 8px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 18px;
  font-weight: 700;
  color: #667eea;
  text-align: center;
}

.current-step {
  background: #f9fafb;
  padding: 15px;
  border-radius: 10px;
  margin-bottom: 15px;
  text-align: center;
}

.current-step h3 {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.logs {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.logs h4 {
  font-size: 14px;
  font-weight: 600;
  color: #333;
  margin-bottom: 10px;
}

.log-content {
  background: #1e293b;
  color: #10b981;
  padding: 15px;
  border-radius: 10px;
  flex: 1;
  overflow-y: auto;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.6;
  scrollbar-width: thin;
  scrollbar-color: #475569 #1e293b;
}

.log-content::-webkit-scrollbar {
  width: 8px;
}

.log-content::-webkit-scrollbar-track {
  background: #1e293b;
  border-radius: 10px;
}

.log-content::-webkit-scrollbar-thumb {
  background: #475569;
  border-radius: 10px;
}

.log-content::-webkit-scrollbar-thumb:hover {
  background: #64748b;
}

.log-content p {
  margin: 3px 0;
  word-break: break-all;
}

.log-content .log-empty {
  color: #64748b;
  font-style: italic;
}
</style>
