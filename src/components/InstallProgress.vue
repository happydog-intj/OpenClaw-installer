<template>
  <div class="install-progress">
    <div class="container">
      <h2>正在安装 OpenClaw</h2>
      <p class="subtitle">请稍候，这可能需要几分钟...</p>
      
      <div class="progress-bar">
        <div 
          class="progress-fill"
          :style="{ width: `${progress.progress}%` }"
        ></div>
      </div>
      <p class="progress-text">{{ Math.round(progress.progress) }}%</p>
      
      <div class="current-step">
        <div class="spinner" v-if="progress.status === 'running'"></div>
        <h3>{{ progress.step }}</h3>
        <p>{{ progress.message }}</p>
      </div>
      
      <div class="logs" v-if="progress.logs && progress.logs.length > 0">
        <h4>详细日志</h4>
        <div class="log-content">
          <p v-for="(log, i) in progress.logs" :key="i">{{ log }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  progress: {
    step: string
    status: string
    progress: number
    message: string
    logs: string[]
  }
}>()
</script>

<style scoped>
.install-progress {
  width: 100%;
  max-width: 700px;
  padding: 40px;
}

.container {
  background: white;
  border-radius: 20px;
  padding: 50px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  text-align: center;
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
  margin-bottom: 40px;
}

.progress-bar {
  width: 100%;
  height: 12px;
  background: #e5e7eb;
  border-radius: 10px;
  overflow: hidden;
  margin-bottom: 10px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 24px;
  font-weight: 700;
  color: #667eea;
  margin-bottom: 30px;
}

.current-step {
  background: #f9fafb;
  padding: 30px;
  border-radius: 15px;
  margin-bottom: 20px;
}

.spinner {
  width: 40px;
  height: 40px;
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

.current-step h3 {
  font-size: 20px;
  font-weight: 600;
  color: #333;
  margin-bottom: 10px;
}

.current-step p {
  font-size: 14px;
  color: #666;
}

.logs {
  text-align: left;
}

.logs h4 {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin-bottom: 10px;
}

.log-content {
  background: #1e293b;
  color: #10b981;
  padding: 15px;
  border-radius: 10px;
  max-height: 200px;
  overflow-y: auto;
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 12px;
}

.log-content p {
  margin: 5px 0;
}
</style>
