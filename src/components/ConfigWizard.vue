<template>
  <div class="config-wizard">
    <!-- Toast 提示 -->
    <div v-if="showToast" class="toast">
      {{ toastMessage }}
    </div>
    
    <div class="container">
      <h2>OpenClaw 配置向导</h2>
      <p class="subtitle">{{ steps[currentStepIndex].description }}</p>
      
      <!-- 现有配置提示 -->
      <div v-if="hasExistingConfig && existingConfigLoaded" class="existing-config-notice">
        <span class="notice-icon">ℹ️</span>
        <span class="notice-text">检测到现有配置，已自动填充</span>
      </div>
      
      <!-- 步骤指示器 -->
      <div class="steps-indicator">
        <div 
          v-for="(step, index) in steps" 
          :key="index"
          class="step-dot"
          :class="{ active: index === currentStepIndex, completed: index < currentStepIndex }"
        >
          <span>{{ index + 1 }}</span>
        </div>
      </div>
      
      <!-- 步骤内容 -->
      <div class="step-content">
        <!-- 步骤 0: 工作目录 -->
        <div v-if="currentStepIndex === 0" class="workspace-selection">
          <div class="input-group">
            <label>工作目录</label>
            <div class="path-input">
              <input 
                v-model="config.workspace"
                type="text"
                placeholder="~/clawd"
              />
              <button @click="browseWorkspace" class="btn-browse">
                📁 浏览
              </button>
            </div>
            <p class="hint">
              OpenClaw 会在这个目录中保存你的 agent 文件和配置
            </p>
          </div>
        </div>
        
        <!-- 步骤 1: API Keys -->
        <div v-if="currentStepIndex === 1" class="api-keys">
          <p class="section-intro">
            配置 AI 模型的 API Keys（可选，稍后也可以配置）
          </p>
          
          <!-- 已配置的模型信息 -->
          <div v-if="modelsList.length > 0" class="configured-models-summary">
            <h4>🎯 当前已配置的模型</h4>
            <div class="models-grid">
              <div v-for="model in modelsList.slice(0, 6)" :key="model.id" class="model-card">
                <div class="model-provider">{{ model.provider }}</div>
                <div class="model-name">{{ model.name }}</div>
              </div>
            </div>
            <p v-if="modelsList.length > 6" class="more-models">
              + 其他 {{ modelsList.length - 6 }} 个模型
            </p>
          </div>
          
          <div class="provider-list">
            <div 
              v-for="provider in providers" 
              :key="provider.id"
              class="provider-item"
            >
              <div class="provider-header" @click="toggleProvider(provider.id)">
                <div class="provider-info">
                  <span class="provider-icon">{{ provider.icon }}</span>
                  <span class="provider-name">{{ provider.name }}</span>
                </div>
                <span class="toggle-icon">{{ expandedProviders[provider.id] ? '▼' : '▶' }}</span>
              </div>
              
              <div v-if="expandedProviders[provider.id]" class="provider-config">
                <!-- 显示已配置的 API Key 信息 -->
                <div v-if="config.apiKeys[provider.id]" class="existing-key-info">
                  <div class="key-preview">
                    <span class="key-label">当前 API Key:</span>
                    <code class="key-value">{{ maskApiKey(config.apiKeys[provider.id]) }}</code>
                  </div>
                </div>
                
                <div class="input-group">
                  <label>
                    {{ config.apiKeys[provider.id] ? '更新 API Key' : 'API Key' }}
                    <span v-if="config.apiKeys[provider.id]" class="configured-badge">✓ 已配置</span>
                  </label>
                  <input 
                    v-model="config.apiKeys[provider.id]"
                    type="password"
                    :placeholder="config.apiKeys[provider.id] ? '留空保持不变' : `输入 ${provider.name} API Key`"
                  />
                </div>
                <a :href="provider.link" target="_blank" class="get-key-link">
                  → 获取 API Key
                </a>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 步骤 2: Bot 配置 -->
        <div v-if="currentStepIndex === 2" class="bot-config">
          <p class="section-intro">
            配置飞书机器人，让 OpenClaw 接入你的飞书工作空间（可选，稍后也可以配置）
          </p>
          
          <div class="bot-platforms">
            <!-- 飞书 Bot -->
            <div class="bot-platform-card bot-platform-card-single">
              <div class="bot-platform-header">
                <span class="platform-icon">🦜</span>
                <h3>飞书 / Lark</h3>
              </div>
              
              <p class="platform-desc">将 OpenClaw 接入飞书，无需公网 IP，支持图片、文件、流式输出</p>
              
              <div class="platform-actions">
                <button class="btn-info" @click="showFeishuGuide = !showFeishuGuide">
                  {{ showFeishuGuide ? '隐藏配置指南' : '📖 查看配置指南' }}
                </button>
              </div>
              
              <div v-if="showFeishuGuide" class="feishu-guide">
                <div class="guide-content">
                  <h4>🚀 新手教程：从零配置飞书 AI 机器人</h4>
                  <p class="guide-note">预计耗时：15–20 分钟</p>
                  
                  <!-- App ID 输入框 -->
                  <div class="app-id-input-section">
                    <label for="feishu-app-id">
                      <strong>📝 填写你的 App ID（可选，用于快速跳转）</strong>
                    </label>
                    <input 
                      id="feishu-app-id"
                      v-model="feishuAppId"
                      type="text"
                      placeholder="例如：cli_a1b2c3d4e5f6g7h8"
                      class="feishu-app-id-input"
                    />
                    <p class="hint">填写后，下方每步的"打开配置页面"按钮会直接跳转到对应页面</p>
                  </div>
                  
                  <div class="guide-step">
                    <div class="step-header">
                      <h5>第一步：创建飞书应用（机器人）</h5>
                      <a href="https://open.feishu.cn/app" target="_blank" class="btn-jump">
                        🚀 打开飞书开放平台
                      </a>
                    </div>
                    <ul>
                      <li>用飞书账号登录</li>
                      <li>点击"创建企业自建应用"</li>
                      <li>填写应用名称（如 "我的 AI 助手"）和描述</li>
                      <li>选择一个图标（之后可修改）</li>
                      <li><strong>创建后，复制 App ID 并填写到上方输入框</strong></li>
                    </ul>
                  </div>
                  
                  <div class="guide-step">
                    <div class="step-header">
                      <h5>第二步：启用机器人能力</h5>
                      <a 
                        v-if="feishuAppId" 
                        :href="`https://open.feishu.cn/app/${feishuAppId}/bot`" 
                        target="_blank" 
                        class="btn-jump"
                      >
                        🤖 打开机器人配置
                      </a>
                      <span v-else class="btn-jump-disabled" title="请先填写 App ID">
                        🤖 打开机器人配置
                      </span>
                    </div>
                    <ul>
                      <li>进入你创建的应用</li>
                      <li>左侧菜单：应用能力 > 机器人</li>
                      <li>开启机器人能力，给机器人起个名字</li>
                    </ul>
                  </div>
                  
                  <div class="guide-step">
                    <div class="step-header">
                      <h5>第三步：配置权限</h5>
                      <a 
                        v-if="feishuAppId" 
                        :href="`https://open.feishu.cn/app/${feishuAppId}/auth`" 
                        target="_blank" 
                        class="btn-jump"
                      >
                        🔐 打开权限管理
                      </a>
                      <span v-else class="btn-jump-disabled" title="请先填写 App ID">
                        🔐 打开权限管理
                      </span>
                    </div>
                    <ul>
                      <li>左侧菜单：权限管理 > 批量导入</li>
                      <li>粘贴以下 JSON（一键导入所有需要的权限）：</li>
                    </ul>
                    <div class="code-block">
                      <pre>{{ feishuPermissionsJson }}</pre>
                      <button class="btn-copy" @click="copyToClipboard(feishuPermissionsJson)">📋 复制</button>
                    </div>
                  </div>
                  
                  <div class="guide-step">
                    <div class="step-header">
                      <h5>第四步：配置事件订阅</h5>
                      <a 
                        v-if="feishuAppId" 
                        :href="`https://open.feishu.cn/app/${feishuAppId}/event`" 
                        target="_blank" 
                        class="btn-jump"
                      >
                        📡 打开事件配置
                      </a>
                      <span v-else class="btn-jump-disabled" title="请先填写 App ID">
                        📡 打开事件配置
                      </span>
                    </div>
                    <p class="warning-note">⚠️ 这一步必须在 OpenClaw 网关启动后再做</p>
                    <ul>
                      <li>左侧菜单：事件与回调 > 事件配置</li>
                      <li>请求方式选择：<strong>使用长连接接收事件</strong>（关键！不需要公网服务器）</li>
                      <li>添加事件：搜索 <code>im.message.receive_v1</code>（接收消息），勾选添加</li>
                    </ul>
                  </div>
                  
                  <div class="guide-step">
                    <div class="step-header">
                      <h5>第五步：记下凭证</h5>
                      <a 
                        v-if="feishuAppId" 
                        :href="`https://open.feishu.cn/app/${feishuAppId}/baseinfo`" 
                        target="_blank" 
                        class="btn-jump"
                      >
                        🔑 打开凭证页面
                      </a>
                      <span v-else class="btn-jump-disabled" title="请先填写 App ID">
                        🔑 打开凭证页面
                      </span>
                    </div>
                    <ul>
                      <li>在应用的"凭证与基础信息"页面，复制：</li>
                      <li><strong>App ID</strong>（格式如 cli_xxxxxxxxx）→ 填写到上方输入框</li>
                      <li><strong>App Secret</strong>（妥善保管，不要分享）→ 在第七步填写</li>
                    </ul>
                  </div>
                  
                  <div class="guide-step">
                    <div class="step-header">
                      <h5>第六步：发布应用</h5>
                      <a 
                        v-if="feishuAppId" 
                        :href="`https://open.feishu.cn/app/${feishuAppId}/version`" 
                        target="_blank" 
                        class="btn-jump"
                      >
                        🚢 打开版本管理
                      </a>
                      <span v-else class="btn-jump-disabled" title="请先填写 App ID">
                        🚢 打开版本管理
                      </span>
                    </div>
                    <ul>
                      <li>左侧菜单：版本管理与发布</li>
                      <li>创建版本 → 填写版本说明 → 提交</li>
                      <li>等待审批（企业内部应用通常自动通过，几秒到几分钟）</li>
                    </ul>
                  </div>
                  
                  <div class="guide-step">
                    <h5>第七步：在 OpenClaw 中配置飞书</h5>
                    
                    <!-- 直接在界面配置 -->
                    <div class="feishu-config-form">
                      <p class="form-intro">✨ 可以直接在这里填写凭证并一键配置：</p>
                      
                      <div class="input-group">
                        <label>App ID</label>
                        <input 
                          v-model="feishuAppId"
                          type="text"
                          placeholder="cli_xxxxxxxxx"
                          :disabled="isInstallingFeishu"
                        />
                      </div>
                      
                      <div class="input-group">
                        <label>App Secret</label>
                        <input 
                          v-model="feishuAppSecret"
                          type="password"
                          placeholder="输入 App Secret"
                          :disabled="isInstallingFeishu"
                        />
                      </div>
                      
                      <button 
                        @click="installAndConfigureFeishu"
                        :disabled="!feishuAppId || !feishuAppSecret || isInstallingFeishu"
                        class="btn-install-feishu"
                      >
                        {{ isInstallingFeishu ? '⏳ 正在配置...' : '🚀 一键安装并配置飞书' }}
                      </button>
                      
                      <!-- 安装日志 -->
                      <div v-if="feishuInstallLog.length > 0" class="install-log">
                        <h6>📋 安装日志：</h6>
                        <div class="log-content">
                          <div v-for="(log, index) in feishuInstallLog" :key="index" class="log-line">
                            {{ log }}
                          </div>
                        </div>
                      </div>
                    </div>
                    
                    <div class="form-divider">
                      <span>或者</span>
                    </div>
                    
                    <!-- 手动命令 -->
                    <div class="manual-commands">
                      <p class="manual-intro">💻 也可以手动在终端运行以下命令：</p>
                      <div class="code-block">
                        <pre>{{ feishuSetupCommands }}</pre>
                        <button class="btn-copy" @click="copyToClipboard(feishuSetupCommands)">📋 复制</button>
                      </div>
                    </div>
                  </div>
                  
                  <div class="guide-step">
                    <h5>第八步：发消息测试</h5>
                    <ul>
                      <li>在飞书里搜索你的机器人名字，打开对话</li>
                      <li>发一条消息，如 "你好"</li>
                      <li>如果机器人回复了配对码，在终端运行：<code>openclaw pairing approve feishu &lt;配对码&gt;</code></li>
                      <li>授权后再发一条消息，收到正常回复 = 配置完成 🎉</li>
                    </ul>
                  </div>
                  
                  <div class="guide-footer">
                    <p><strong>💡 提示：</strong> 完整配置文档请参考：<a href="https://github.com/AlexAnys/openclaw-feishu" target="_blank">github.com/AlexAnys/openclaw-feishu</a></p>
                    <p><strong>🔧 常见问题：</strong> 遇到问题查看 <a href="https://github.com/AlexAnys/openclaw-feishu#-%E5%B8%B8%E8%A7%81%E9%97%AE%E9%A2%98--%E6%8E%92%E6%9F%A5%E6%B8%85%E5%8D%95" target="_blank">常见问题 & 排查清单</a></p>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <div class="other-platforms-note">
            <p>💡 <strong>提示：</strong>OpenClaw 还支持 Telegram、Discord、Slack、WhatsApp、iMessage 等平台，可在安装后通过命令行配置。</p>
            <p>文档：<a href="https://docs.openclaw.ai/channels" target="_blank">https://docs.openclaw.ai/channels</a></p>
          </div>
        </div>
        
        <!-- 最后一步: 确认 -->
        <div v-if="isLastStep" class="confirmation">
          <div class="summary">
            <h3>配置摘要</h3>
            <div class="summary-item">
              <strong>运行模式：</strong>
              <span>本地模式 💻</span>
            </div>
            <div class="summary-item">
              <strong>工作目录：</strong>
              <span>{{ config.workspace || '~/clawd' }}</span>
            </div>
            <div class="summary-item">
              <strong>API Keys：</strong>
              <span>{{ configuredProvidersCount }} 个已配置</span>
            </div>
          </div>
          
          <div class="ready-message">
            <div class="icon">🎉</div>
            <h3>一切就绪！</h3>
            <p>点击"完成配置"开始使用 OpenClaw</p>
          </div>
        </div>
      </div>
      
      <!-- 导航按钮 -->
      <div class="navigation">
        <button 
          v-if="currentStepIndex > 0"
          @click="prevStep"
          class="btn-secondary"
        >
          ← 上一步
        </button>
        <div v-else></div>
        
        <button 
          v-if="!isLastStep"
          @click="nextStep"
          :disabled="!canProceed"
          class="btn-primary"
        >
          下一步 →
        </button>
        <button 
          v-else
          @click="finishConfig"
          class="btn-primary"
        >
          完成配置 ✓
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'

const emit = defineEmits(['complete'])

interface ConfigData {
  mode: 'local' | 'remote'
  workspace: string
  remoteUrl: string
  remoteToken: string
  apiKeys: Record<string, string>
}

const config = reactive<ConfigData>({
  mode: 'local', // 固定为本地模式
  workspace: '~/clawd',
  remoteUrl: '',
  remoteToken: '',
  apiKeys: {}
})

const currentStepIndex = ref(0)
const expandedProviders = reactive<Record<string, boolean>>({})
const existingConfigLoaded = ref(false)
const hasExistingConfig = ref(false)
const configuredModels = ref<Record<string, any>>({})
const modelsList = ref<any[]>([])
const showFeishuGuide = ref(true)
const feishuAppId = ref('')
const feishuAppSecret = ref('')
const isInstallingFeishu = ref(false)
const feishuInstallLog = ref<string[]>([])

// 飞书权限 JSON
const feishuPermissionsJson = JSON.stringify({
  "scopes": {
    "tenant": [
      "aily:file:read",
      "aily:file:write",
      "application:application.app_message_stats.overview:readonly",
      "application:application:self_manage",
      "application:bot.menu:write",
      "cardkit:card:write",
      "contact:user.employee_id:readonly",
      "corehr:file:download",
      "docs:document.content:read",
      "event:ip_list",
      "im:chat",
      "im:chat.access_event.bot_p2p_chat:read",
      "im:chat.members:bot_access",
      "im:message",
      "im:message.group_at_msg:readonly",
      "im:message.group_msg",
      "im:message.p2p_msg:readonly",
      "im:message:readonly",
      "im:message:send_as_bot",
      "im:resource",
      "sheets:spreadsheet",
      "wiki:wiki:readonly"
    ],
    "user": [
      "aily:file:read",
      "aily:file:write",
      "im:chat.access_event.bot_p2p_chat:read"
    ]
  }
}, null, 2)

// 飞书设置命令
const feishuSetupCommands = `# 1. 安装飞书插件
openclaw plugins install @openclaw/feishu

# 2. 添加飞书渠道（交互式引导）
openclaw channels add
# → 选择 Feishu
# → 粘贴 App ID
# → 粘贴 App Secret

# 3. 重启网关
openclaw gateway restart

# 4. 查看日志，确认连接成功
openclaw logs --follow`

const steps = computed(() => {
  return [
    { name: '工作目录', description: '设置 agent 文件的存储位置' },
    { name: 'API Keys', description: '配置 AI 模型提供商（可选）' },
    { name: 'Bot 配置', description: '配置聊天平台机器人（可选）' },
    { name: '完成', description: '确认配置并开始使用' }
  ]
})

const providers = [
  { id: 'qwen', name: 'Qwen (通义千问)', icon: '🌟', link: 'https://dashscope.aliyun.com/' },
  { id: 'kimi', name: 'Kimi (月之暗面)', icon: '🌙', link: 'https://platform.moonshot.cn/console/api-keys' },
  { id: 'minimax', name: 'MiniMax', icon: '⚡', link: 'https://www.minimaxi.com/user-center/basic-information/interface-key' },
  { id: 'zhipu', name: '智谱 (GLM)', icon: '🧊', link: 'https://open.bigmodel.cn/usercenter/apikeys' },
  { id: 'anthropic', name: 'Anthropic (Claude)', icon: '🤖', link: 'https://console.anthropic.com/' },
  { id: 'openai', name: 'OpenAI (GPT)', icon: '🧠', link: 'https://platform.openai.com/api-keys' },
  { id: 'google', name: 'Google (Gemini)', icon: '🔮', link: 'https://makersuite.google.com/app/apikey' },
  { id: 'deepseek', name: 'DeepSeek', icon: '🔍', link: 'https://platform.deepseek.com/api_keys' }
]

const isLastStep = computed(() => currentStepIndex.value === steps.value.length - 1)

const canProceed = computed(() => {
  if (currentStepIndex.value === 0) {
    // 工作目录步骤
    return config.workspace.trim() !== ''
  }
  // API Keys 和 Bot 配置都是可选的，可以直接跳过
  return true
})

const configuredProvidersCount = computed(() => {
  return Object.values(config.apiKeys).filter(key => key.trim() !== '').length
})

function toggleProvider(id: string) {
  expandedProviders[id] = !expandedProviders[id]
}

function maskApiKey(key: string): string {
  if (!key) return ''
  if (key.length <= 8) return '••••••••'
  
  // 显示前4个字符和后4个字符，中间用点号遮蔽
  const start = key.substring(0, 4)
  const end = key.substring(key.length - 4)
  const middle = '•'.repeat(Math.min(20, key.length - 8))
  
  return `${start}${middle}${end}`
}

// Toast 状态
const showToast = ref(false)
const toastMessage = ref('')

function showToastMessage(message: string) {
  toastMessage.value = message
  showToast.value = true
  setTimeout(() => {
    showToast.value = false
  }, 2000)
}

async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    showToastMessage('✓ 已复制到剪贴板')
  } catch (error) {
    console.error('复制失败:', error)
    showToastMessage('✗ 复制失败，请手动复制')
  }
}

async function browseWorkspace() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: config.workspace || '~/'
    })
    
    if (selected && typeof selected === 'string') {
      config.workspace = selected
    }
  } catch (error) {
    console.error('选择目录失败:', error)
  }
}

function nextStep() {
  if (canProceed.value && currentStepIndex.value < steps.value.length - 1) {
    currentStepIndex.value++
  }
}

function prevStep() {
  if (currentStepIndex.value > 0) {
    currentStepIndex.value--
  }
}

async function loadExistingConfig() {
  try {
    const existing = await invoke('load_existing_config') as any
    
    if (existing.exists) {
      hasExistingConfig.value = true
      
      // 预填充配置
      if (existing.mode) {
        config.mode = existing.mode
      }
      if (existing.workspace) {
        config.workspace = existing.workspace
      }
      if (existing.remoteUrl) {
        config.remoteUrl = existing.remoteUrl
      }
      if (existing.apiKeys && typeof existing.apiKeys === 'object') {
        Object.assign(config.apiKeys, existing.apiKeys)
        
        // 展开已配置的 providers
        for (const provider in existing.apiKeys) {
          if (existing.apiKeys[provider]) {
            expandedProviders[provider] = true
          }
        }
      }
      
      // 保存配置的模型信息
      if (existing.configuredModels) {
        configuredModels.value = existing.configuredModels
      }
      
      // 保存模型列表
      if (existing.models) {
        modelsList.value = existing.models
      }
    }
    
    existingConfigLoaded.value = true
  } catch (error) {
    console.error('加载现有配置失败:', error)
    existingConfigLoaded.value = true
  }
}

async function finishConfig() {
  try {
    // 调用后端保存配置
    await invoke('save_config', { config })
    emit('complete')
  } catch (error) {
    console.error('保存配置失败:', error)
    alert(`配置失败: ${error}`)
  }
}

async function installAndConfigureFeishu() {
  if (!feishuAppId.value || !feishuAppSecret.value) {
    showToastMessage('✗ 请填写 App ID 和 App Secret')
    return
  }
  
  isInstallingFeishu.value = true
  feishuInstallLog.value = []
  
  try {
    feishuInstallLog.value.push('🔄 开始安装飞书插件...')
    
    // 调用后端执行安装命令
    const result = await invoke('install_feishu_plugin', {
      appId: feishuAppId.value,
      appSecret: feishuAppSecret.value
    }) as any
    
    feishuInstallLog.value.push(...result.logs)
    
    if (result.success) {
      feishuInstallLog.value.push('✅ 飞书配置完成！')
      showToastMessage('✓ 飞书配置成功')
    } else {
      feishuInstallLog.value.push(`❌ 配置失败: ${result.error}`)
      showToastMessage('✗ 配置失败，请查看日志')
    }
  } catch (error) {
    console.error('安装飞书失败:', error)
    feishuInstallLog.value.push(`❌ 错误: ${error}`)
    showToastMessage('✗ 配置失败')
  } finally {
    isInstallingFeishu.value = false
  }
}

onMounted(() => {
  loadExistingConfig()
})
</script>

<style scoped>
.config-wizard {
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
  padding: 20px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  width: 100%;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
}

h2 {
  font-size: 14px;
  font-weight: 700;
  color: #333;
  margin-bottom: 5px;
  text-align: center;
}

.subtitle {
  font-size: 8px;
  color: #666;
  text-align: center;
  margin-bottom: 10px;
}

.existing-config-notice {
  background: #eff6ff;
  border: 1px solid #3b82f6;
  border-radius: 5px;
  padding: 6px 10px;
  margin-bottom: 10px;
  display: flex;
  align-items: center;
  gap: 5px;
}

.notice-icon {
  font-size: 10px;
}

.notice-text {
  font-size: 7px;
  color: #1e40af;
  font-weight: 500;
}

.steps-indicator {
  display: flex;
  justify-content: center;
  gap: 8px;
  margin-bottom: 20px;
}

.step-dot {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: #e5e7eb;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 10px;
  color: #9ca3af;
  transition: all 0.3s;
}

.step-dot.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  transform: scale(1.2);
}

.step-dot.completed {
  background: #10b981;
  color: white;
}

.step-content {
  flex: 1;
  overflow-y: auto;
  margin-bottom: 15px;
  min-height: 400px;
}

.mode-selection {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
}

.mode-card {
  padding: 30px;
  border: 3px solid #e5e7eb;
  border-radius: 15px;
  cursor: pointer;
  transition: all 0.3s;
  text-align: center;
}

.mode-card:hover {
  border-color: #667eea;
  transform: translateY(-5px);
  box-shadow: 0 10px 30px rgba(102, 126, 234, 0.2);
}

.mode-card.selected {
  border-color: #667eea;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
}

.mode-card .icon {
  font-size: 48px;
  margin-bottom: 15px;
}

.mode-card h3 {
  font-size: 20px;
  font-weight: 700;
  color: #333;
  margin-bottom: 10px;
}

.mode-card p {
  font-size: 14px;
  color: #666;
  margin-bottom: 15px;
}

.mode-card ul {
  text-align: left;
  list-style: none;
  padding: 0;
}

.mode-card ul li {
  font-size: 13px;
  color: #666;
  margin: 5px 0;
}

.workspace-selection,
.remote-config,
.api-keys {
  max-width: 600px;
  margin: 0 auto;
}

.input-group {
  margin-bottom: 25px;
}

.input-group label {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 14px;
  font-weight: 600;
  color: #333;
  margin-bottom: 8px;
}

.configured-badge {
  font-size: 12px;
  color: #10b981;
  font-weight: 500;
  background: #d1fae5;
  padding: 2px 8px;
  border-radius: 4px;
}

.input-group input {
  width: 100%;
  padding: 12px 15px;
  font-size: 15px;
  border: 2px solid #e5e7eb;
  border-radius: 8px;
  transition: border-color 0.2s;
}

.input-group input:focus {
  outline: none;
  border-color: #667eea;
}

.path-input {
  display: flex;
  gap: 10px;
}

.path-input input {
  flex: 1;
}

.btn-browse {
  padding: 12px 20px;
  background: #f3f4f6;
  border: 2px solid #e5e7eb;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.btn-browse:hover {
  background: #e5e7eb;
}

.hint {
  font-size: 13px;
  color: #9ca3af;
  margin-top: 8px;
}

.section-intro {
  font-size: 7.5px;
  color: #666;
  margin-bottom: 10px;
  text-align: center;
}

.provider-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.provider-item {
  border: 2px solid #e5e7eb;
  border-radius: 10px;
  overflow: hidden;
}

.provider-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  cursor: pointer;
  transition: background 0.2s;
}

.provider-header:hover {
  background: #f9fafb;
}

.provider-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.provider-icon {
  font-size: 24px;
}

.provider-name {
  font-size: 16px;
  font-weight: 600;
  color: #333;
}

.toggle-icon {
  font-size: 12px;
  color: #9ca3af;
}

.provider-config {
  padding: 0 20px 20px;
  background: #f9fafb;
}

.get-key-link {
  display: inline-block;
  font-size: 13px;
  color: #667eea;
  text-decoration: none;
  margin-top: 5px;
}

.get-key-link:hover {
  text-decoration: underline;
}

.configured-models-summary {
  background: #f0f9ff;
  border: 2px solid #3b82f6;
  border-radius: 10px;
  padding: 20px;
  margin-bottom: 25px;
}

.configured-models-summary h4 {
  font-size: 16px;
  font-weight: 700;
  color: #1e40af;
  margin-bottom: 15px;
}

.models-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 10px;
  margin-bottom: 10px;
}

.model-card {
  background: white;
  border: 1px solid #bfdbfe;
  border-radius: 8px;
  padding: 12px;
}

.model-provider {
  font-size: 11px;
  color: #3b82f6;
  font-weight: 600;
  text-transform: uppercase;
  margin-bottom: 4px;
}

.model-name {
  font-size: 13px;
  color: #1e40af;
  font-weight: 500;
}

.more-models {
  font-size: 13px;
  color: #6b7280;
  text-align: center;
  margin-top: 10px;
}

.existing-key-info {
  background: #f9fafb;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  padding: 12px;
  margin-bottom: 15px;
}

.key-preview {
  display: flex;
  align-items: center;
  gap: 10px;
}

.key-label {
  font-size: 13px;
  color: #6b7280;
  font-weight: 500;
}

.key-value {
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 12px;
  color: #374151;
  background: white;
  padding: 4px 8px;
  border-radius: 4px;
  border: 1px solid #d1d5db;
}

.confirmation {
  max-width: 600px;
  margin: 0 auto;
}

.summary {
  background: #f9fafb;
  padding: 25px;
  border-radius: 10px;
  margin-bottom: 30px;
}

.summary h3 {
  font-size: 18px;
  font-weight: 700;
  color: #333;
  margin-bottom: 20px;
}

.summary-item {
  display: flex;
  justify-content: space-between;
  padding: 12px 0;
  border-bottom: 1px solid #e5e7eb;
}

.summary-item:last-child {
  border-bottom: none;
}

.summary-item strong {
  color: #666;
  font-weight: 600;
}

.summary-item span {
  color: #333;
}

.ready-message {
  text-align: center;
  padding: 30px;
}

.ready-message .icon {
  font-size: 64px;
  margin-bottom: 20px;
}

.ready-message h3 {
  font-size: 24px;
  font-weight: 700;
  color: #333;
  margin-bottom: 10px;
}

.ready-message p {
  font-size: 16px;
  color: #666;
}

.navigation {
  display: flex;
  justify-content: space-between;
  gap: 15px;
  padding-top: 10px;
  border-top: 1px solid #e5e7eb;
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
  max-width: 200px;
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

/* Bot 配置样式 */
.bot-config {
  padding: 0;
}

.bot-platforms {
  display: flex;
  flex-direction: column;
  gap: 20px;
  margin-bottom: 25px;
}

.bot-platform-card {
  background: white;
  border: 2px solid #e5e7eb;
  border-radius: 12px;
  padding: 20px;
  transition: all 0.3s;
}

.bot-platform-card:hover {
  border-color: #667eea;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.1);
}

.bot-platform-card-single {
  max-width: 100%;
  margin: 0 auto;
}

.other-platforms-note {
  background: #f0f9ff;
  border: 1px solid #bfdbfe;
  border-radius: 10px;
  padding: 15px 20px;
  margin-top: 20px;
}

.other-platforms-note p {
  font-size: 13px;
  color: #1e40af;
  line-height: 1.6;
  margin: 5px 0;
}

.other-platforms-note a {
  color: #2563eb;
  text-decoration: none;
  font-weight: 600;
}

.other-platforms-note a:hover {
  text-decoration: underline;
}

.bot-platform-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.platform-icon {
  font-size: 32px;
}

.bot-platform-header h3 {
  font-size: 20px;
  font-weight: 700;
  color: #333;
  margin: 0;
}

.platform-desc {
  color: #666;
  font-size: 14px;
  margin-bottom: 15px;
  line-height: 1.6;
}

.platform-actions {
  display: flex;
  gap: 10px;
}

.btn-info {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 10px 20px;
  font-size: 14px;
  font-weight: 600;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-info:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 15px rgba(102, 126, 234, 0.3);
}

.feishu-guide {
  margin-top: 20px;
  border-top: 2px solid #e5e7eb;
  padding-top: 20px;
}

.guide-content {
  max-height: 500px;
  overflow-y: auto;
  padding-right: 10px;
}

.guide-content::-webkit-scrollbar {
  width: 6px;
}

.guide-content::-webkit-scrollbar-track {
  background: #f3f4f6;
  border-radius: 10px;
}

.guide-content::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 10px;
}

.guide-content h4 {
  font-size: 18px;
  font-weight: 700;
  color: #333;
  margin-bottom: 10px;
}

.guide-note {
  color: #666;
  font-size: 13px;
  margin-bottom: 20px;
  font-style: italic;
}

.guide-step {
  margin-bottom: 25px;
  padding: 15px;
  background: #f9fafb;
  border-radius: 8px;
  border-left: 4px solid #667eea;
}

.guide-step h5 {
  font-size: 16px;
  font-weight: 700;
  color: #667eea;
  margin-bottom: 10px;
}

.guide-step ul {
  margin: 10px 0;
  padding-left: 25px;
}

.guide-step li {
  margin-bottom: 6px;
  color: #374151;
  font-size: 14px;
  line-height: 1.6;
}

.guide-step code {
  background: #e5e7eb;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 13px;
  color: #d63384;
}

.guide-step a {
  color: #667eea;
  text-decoration: none;
  font-weight: 600;
}

.guide-step a:hover {
  text-decoration: underline;
}

.warning-note {
  background: #fef2f2;
  color: #dc2626;
  padding: 8px 12px;
  border-radius: 6px;
  font-size: 13px;
  margin-bottom: 10px;
  border-left: 3px solid #ef4444;
}

.code-block {
  position: relative;
  background: #1e293b;
  border-radius: 8px;
  padding: 15px;
  margin: 10px 0;
  overflow-x: auto;
}

.code-block pre {
  margin: 0;
  color: #e2e8f0;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 12px;
  line-height: 1.6;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.btn-copy {
  position: absolute;
  top: 10px;
  right: 10px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.2);
  padding: 6px 12px;
  font-size: 12px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-copy:hover {
  background: rgba(255, 255, 255, 0.2);
}

.guide-footer {
  margin-top: 20px;
  padding-top: 15px;
  border-top: 2px solid #e5e7eb;
  font-size: 13px;
  color: #666;
  line-height: 1.8;
}

.guide-footer a {
  color: #667eea;
  text-decoration: none;
  font-weight: 600;
}

.guide-footer a:hover {
  text-decoration: underline;
}

/* Toast 样式 */
.toast {
  position: fixed;
  top: 30px;
  right: 30px;
  background: #10b981;
  color: white;
  padding: 15px 25px;
  border-radius: 10px;
  box-shadow: 0 10px 30px rgba(16, 185, 129, 0.3);
  font-size: 15px;
  font-weight: 600;
  z-index: 10000;
  animation: slideInRight 0.3s ease-out;
}

@keyframes slideInRight {
  from {
    transform: translateX(100px);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

/* App ID 输入框样式 */
.app-id-input-section {
  background: #eff6ff;
  border: 2px solid #3b82f6;
  border-radius: 10px;
  padding: 20px;
  margin-bottom: 25px;
}

.app-id-input-section label {
  display: block;
  color: #1e40af;
  margin-bottom: 10px;
  font-size: 14px;
}

.feishu-app-id-input {
  width: 100%;
  padding: 12px 15px;
  font-size: 15px;
  border: 2px solid #3b82f6;
  border-radius: 8px;
  font-family: 'Monaco', 'Menlo', monospace;
  transition: border-color 0.2s;
}

.feishu-app-id-input:focus {
  outline: none;
  border-color: #2563eb;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.app-id-input-section .hint {
  font-size: 12px;
  color: #3b82f6;
  margin-top: 8px;
}

/* 步骤头部样式 */
.step-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  flex-wrap: wrap;
  gap: 10px;
}

.step-header h5 {
  margin: 0;
}

.btn-jump {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 600;
  text-decoration: none;
  transition: all 0.2s;
  white-space: nowrap;
}

.btn-jump:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 15px rgba(102, 126, 234, 0.3);
}

.btn-jump-disabled {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background: #cbd5e1;
  color: #94a3b8;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 600;
  cursor: not-allowed;
  white-space: nowrap;
}

/* 飞书配置表单样式 */
.feishu-config-form {
  background: #f0f9ff;
  border: 2px solid #3b82f6;
  border-radius: 10px;
  padding: 20px;
  margin-bottom: 20px;
}

.form-intro {
  color: #1e40af;
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 15px;
}

.btn-install-feishu {
  width: 100%;
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  border: none;
  padding: 14px 24px;
  font-size: 16px;
  font-weight: 700;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  margin-top: 10px;
}

.btn-install-feishu:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(16, 185, 129, 0.3);
}

.btn-install-feishu:disabled {
  background: #cbd5e1;
  cursor: not-allowed;
  transform: none;
}

.install-log {
  margin-top: 15px;
  background: white;
  border: 1px solid #3b82f6;
  border-radius: 8px;
  padding: 15px;
}

.install-log h6 {
  font-size: 14px;
  font-weight: 600;
  color: #1e40af;
  margin-bottom: 10px;
}

.log-content {
  max-height: 200px;
  overflow-y: auto;
  background: #1e293b;
  border-radius: 6px;
  padding: 10px;
}

.log-line {
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 12px;
  color: #e2e8f0;
  padding: 3px 0;
  line-height: 1.5;
}

.form-divider {
  position: relative;
  text-align: center;
  margin: 25px 0;
}

.form-divider::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 0;
  right: 0;
  height: 1px;
  background: #e5e7eb;
}

.form-divider span {
  position: relative;
  background: #f9fafb;
  padding: 0 15px;
  color: #9ca3af;
  font-size: 13px;
  font-weight: 600;
}

.manual-commands {
  background: #f9fafb;
  border-radius: 8px;
  padding: 15px;
}

.manual-intro {
  font-size: 14px;
  color: #6b7280;
  margin-bottom: 10px;
  font-weight: 500;
}
</style>
