<template>
  <div class="welcome-screen">
    <div class="container">
      <div class="logo">
        <img src="../assets/openclaw-logo.svg" alt="OpenClaw" />
      </div>
      
      <h1>OpenClaw 安装向导</h1>
      <p class="subtitle">让 AI 助手触手可及</p>
      
      <div class="features">
        <div class="feature">
          <div class="icon">✨</div>
          <h3>一键安装</h3>
          <p>自动检测并安装所有依赖</p>
        </div>
        <div class="feature">
          <div class="icon">🚀</div>
          <h3>快速配置</h3>
          <p>向导式初始化，5 分钟上手</p>
        </div>
        <div class="feature">
          <div class="icon">🔒</div>
          <h3>安全可靠</h3>
          <p>开源透明，隐私优先</p>
        </div>
      </div>
      
      <div class="system-info">
        <p>系统: {{ systemInfo.os }} {{ systemInfo.osVersion }}</p>
        <p>架构: {{ systemInfo.arch }}</p>
      </div>
      
      <button @click="$emit('next')" class="btn-primary">
        开始安装 →
      </button>
      
      <p class="footer-text">
        安装过程需要联网下载约 100MB 数据
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

defineEmits(['next'])

const systemInfo = ref({
  os: '',
  osVersion: '',
  arch: ''
})

onMounted(async () => {
  try {
    const info = await invoke('get_system_info')
    systemInfo.value = info as any
  } catch (error) {
    console.error('获取系统信息失败:', error)
  }
})
</script>

<style scoped>
.welcome-screen {
  width: 100%;
  max-width: 600px;
  padding: 40px;
}

.container {
  background: white;
  border-radius: 20px;
  padding: 50px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  text-align: center;
}

.logo img {
  width: 120px;
  height: 120px;
  margin-bottom: 30px;
}

h1 {
  font-size: 32px;
  font-weight: 700;
  color: #333;
  margin-bottom: 10px;
}

.subtitle {
  font-size: 18px;
  color: #666;
  margin-bottom: 40px;
}

.features {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 30px;
  margin-bottom: 40px;
}

.feature {
  text-align: center;
}

.feature .icon {
  font-size: 40px;
  margin-bottom: 10px;
}

.feature h3 {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin-bottom: 8px;
}

.feature p {
  font-size: 14px;
  color: #666;
}

.system-info {
  background: #f5f5f5;
  padding: 15px;
  border-radius: 10px;
  margin-bottom: 30px;
}

.system-info p {
  font-size: 14px;
  color: #666;
  margin: 5px 0;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 15px 40px;
  font-size: 18px;
  font-weight: 600;
  border-radius: 10px;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 10px 20px rgba(102, 126, 234, 0.4);
}

.footer-text {
  margin-top: 20px;
  font-size: 12px;
  color: #999;
}
</style>
