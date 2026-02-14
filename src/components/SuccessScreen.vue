<template>
  <div class="success-screen">
    <div class="container">
      <div class="success-icon">🎉</div>
      <h1>安装成功！</h1>
      <p class="subtitle">OpenClaw 已成功安装到你的系统</p>
      
      <div class="next-steps">
        <h3>下一步操作</h3>
        <div class="step">
          <div class="step-number">1</div>
          <div class="step-content">
            <h4>打开终端</h4>
            <p>在终端中运行 <code>openclaw</code> 命令</p>
          </div>
        </div>
        <div class="step">
          <div class="step-number">2</div>
          <div class="step-content">
            <h4>配置账号</h4>
            <p>按照向导连接你的 WhatsApp、Telegram 等</p>
          </div>
        </div>
        <div class="step">
          <div class="step-number">3</div>
          <div class="step-content">
            <h4>开始使用</h4>
            <p>享受 AI 助手带来的便利！</p>
          </div>
        </div>
      </div>
      
      <div class="quick-commands">
        <h3>常用命令</h3>
        <div class="command-list">
          <div class="command">
            <code>openclaw status</code>
            <span>查看运行状态</span>
          </div>
          <div class="command">
            <code>openclaw gateway</code>
            <span>启动网关</span>
          </div>
          <div class="command">
            <code>openclaw help</code>
            <span>查看帮助</span>
          </div>
        </div>
      </div>
      
      <div class="actions">
        <button @click="openDocs" class="btn-secondary">
          📖 查看文档
        </button>
        <button @click="$emit('close')" class="btn-primary">
          完成 ✓
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/api/shell'

defineEmits(['close'])

async function openDocs() {
  await open('https://docs.openclaw.ai')
}
</script>

<style scoped>
.success-screen {
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

.success-icon {
  font-size: 100px;
  margin-bottom: 20px;
}

h1 {
  font-size: 36px;
  font-weight: 700;
  color: #333;
  margin-bottom: 10px;
}

.subtitle {
  font-size: 18px;
  color: #666;
  margin-bottom: 40px;
}

.next-steps {
  text-align: left;
  margin-bottom: 30px;
}

.next-steps h3 {
  font-size: 20px;
  font-weight: 600;
  color: #333;
  margin-bottom: 20px;
  text-align: center;
}

.step {
  display: flex;
  gap: 20px;
  margin-bottom: 20px;
  padding: 20px;
  background: #f9fafb;
  border-radius: 10px;
}

.step-number {
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: 700;
  flex-shrink: 0;
}

.step-content h4 {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin-bottom: 5px;
}

.step-content p {
  font-size: 14px;
  color: #666;
}

.step-content code {
  background: #1e293b;
  color: #10b981;
  padding: 2px 8px;
  border-radius: 4px;
  font-family: 'Monaco', monospace;
  font-size: 13px;
}

.quick-commands {
  margin-bottom: 30px;
}

.quick-commands h3 {
  font-size: 20px;
  font-weight: 600;
  color: #333;
  margin-bottom: 15px;
}

.command-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.command {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px;
  background: #f9fafb;
  border-radius: 8px;
}

.command code {
  background: #1e293b;
  color: #10b981;
  padding: 5px 12px;
  border-radius: 6px;
  font-family: 'Monaco', monospace;
  font-size: 14px;
}

.command span {
  font-size: 14px;
  color: #666;
}

.actions {
  display: flex;
  gap: 15px;
}

.btn-secondary {
  background: #e5e7eb;
  color: #374151;
  border: none;
  padding: 15px 30px;
  font-size: 16px;
  font-weight: 600;
  border-radius: 10px;
  cursor: pointer;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 15px 30px;
  font-size: 16px;
  font-weight: 600;
  border-radius: 10px;
  cursor: pointer;
  flex: 1;
}
</style>
