<template>
  <div class="settings flex flex-col h-full">
    <div class="flex-1 bg-white p-16px rounded-8px overflow-hidden">
      <el-tabs v-model="activeTab">
        <el-tab-pane label="用户管理" name="users">
          <div class="flex flex-col gap-4">
            <div class="flex justify-between items-center mb-4">
              <div class="text-lg font-medium">用户管理</div>
              <div>
                <el-button type="primary" size="small" @click="openAddUser">新增用户</el-button>
              </div>
            </div>

            <el-table :data="usersPaginated" stripe style="width:100%">
              <el-table-column type="index" label="序号" width="80" />
              <el-table-column prop="employeeId" label="工号" width="140" />
              <el-table-column prop="name" label="姓名" width="160" />
              <el-table-column prop="team" label="班组" width="160" />
              <el-table-column prop="role" label="角色"  />
              <el-table-column label="操作" width="300">
                <template #default="{ row, $index }">
                  <el-button size="small" @click="openEditUser(row, $index)">修改</el-button>
                  <el-button size="small" type="warning" @click="resetPassword($index)" :disabled="currentUserRole !== '管理员'">重置密码</el-button>
                  <el-button size="small" type="danger" @click="deleteUser($index)" :disabled="currentUserRole !== '管理员'">删除</el-button>
                </template>
              </el-table-column>
            </el-table>

            <div class="pagination-wrap" style="display:flex;justify-content:flex-end;margin-top:12px;">
              <el-pagination
                background
                layout="total, sizes, prev, pager, next, jumper"
                :total="users.length"
                :page-size="pageSize"
                :current-page.sync="currentPage"
                :page-sizes="[5,10,20]"
                @size-change="handleSizeChange"
                @current-change="handlePageChange"
              />
            </div>
          </div>
        </el-tab-pane>

        <el-tab-pane label="系统设置" name="system">
          <div class="system-settings">
            <el-tabs v-model="systemTab" type="border-card">
              <el-tab-pane label="基础配置" name="basic">
                <el-form :model="basicConfig" label-width="120px" class="p-4">
                  <div class="mb-4">
                    <div class="text-md font-medium mb-2">数据库</div>
                    <el-form-item label="主机">
                      <el-input v-model="basicConfig.db.host" placeholder="127.0.0.1" />
                    </el-form-item>
                    <el-form-item label="端口">
                      <el-input v-model="basicConfig.db.port" />
                    </el-form-item>
                    <el-form-item label="用户名">
                      <el-input v-model="basicConfig.db.user" />
                    </el-form-item>
                    <el-form-item label="密码">
                      <el-input v-model="basicConfig.db.password" show-password />
                    </el-form-item>
                    <el-form-item label="数据库名">
                      <el-input v-model="basicConfig.db.database" />
                    </el-form-item>
                  </div>

                  <div class="mb-4">
                    <div class="text-md font-medium mb-2">日志</div>
                    <el-form-item label="日志等级">
                      <el-select v-model="basicConfig.log.level">
                        <el-option label="DEBUG" value="DEBUG" />
                        <el-option label="INFO" value="INFO" />
                        <el-option label="WARN" value="WARN" />
                        <el-option label="ERROR" value="ERROR" />
                      </el-select>
                    </el-form-item>
                    <el-form-item label="轮转大小(MB)">
                      <el-input v-model.number="basicConfig.log.maxSize" />
                    </el-form-item>
                  </div>

                  <div class="mb-4">
                    <div class="text-md font-medium mb-2">数据存储目录</div>
                    <el-form-item label="目录路径">
                      <el-input v-model="basicConfig.storage.path" placeholder="E:/data/storage" />
                    </el-form-item>
                    <el-form-item>
                      <el-button size="small" @click="chooseStorageDir">选择目录</el-button>
                    </el-form-item>
                  </div>

                  <el-form-item>
                    <el-button type="primary" @click="saveBasicConfig">保存基础配置</el-button>
                  </el-form-item>
                </el-form>
              </el-tab-pane>

              <el-tab-pane label="硬件管理" name="hardware">
                <div class="p-4">
                  <div class="mb-4">
                    <div class="text-md font-medium mb-2">PLC</div>
                    <el-form :model="plcConfig" label-width="120px">
                      <el-form-item label="IP 地址">
                        <el-input v-model="plcConfig.ip" />
                      </el-form-item>
                      <el-form-item label="端口">
                        <el-input v-model.number="plcConfig.port" />
                      </el-form-item>
                      <el-form-item>
                        <el-button type="primary" size="small" @click="savePlc">保存PLC配置</el-button>
                      </el-form-item>
                    </el-form>
                  </div>

                  <div class="mb-4">
                    <div class="flex justify-between items-center mb-2">
                      <div class="text-md font-medium">相机（串口）</div>
                      <el-button size="small" type="primary" @click="openAddCamera">新增相机</el-button>
                    </div>
                    <el-table :data="cameras" stripe style="width:100%">
                      <el-table-column type="index" label="序号" width="80" />
                      <el-table-column prop="name" label="名称" />
                      <el-table-column prop="port" label="串口" width="140" />
                      <el-table-column label="操作" width="160">
                        <template #default="{ row, $index }">
                          <el-button size="small" @click="openEditCamera(row, $index)">编辑</el-button>
                          <el-button size="small" type="danger" @click="removeCamera($index)">删除</el-button>
                        </template>
                      </el-table-column>
                    </el-table>
                  </div>

                  <div class="mb-4">
                    <div class="flex justify-between items-center mb-2">
                      <div class="text-md font-medium">光源（串口）</div>
                      <el-button size="small" type="primary" @click="openAddLight">新增光源</el-button>
                    </div>
                    <el-table :data="lights" stripe style="width:100%">
                      <el-table-column type="index" label="序号" width="80" />
                      <el-table-column prop="name" label="名称" />
                      <el-table-column prop="port" label="串口" width="140" />
                      <el-table-column label="操作" width="160">
                        <template #default="{ row, $index }">
                          <el-button size="small" @click="openEditLight(row, $index)">编辑</el-button>
                          <el-button size="small" type="danger" @click="removeLight($index)">删除</el-button>
                        </template>
                      </el-table-column>
                    </el-table>
                  </div>

                  <div class="mb-4">
                    <div class="text-md font-medium mb-2">扫码枪（串口）</div>
                    <el-form :model="scannerConfig" label-width="120px">
                      <el-form-item label="串口">
                        <el-input v-model="scannerConfig.port" />
                      </el-form-item>
                      <el-form-item>
                        <el-button type="primary" size="small" @click="saveScanner">保存扫码枪</el-button>
                      </el-form-item>
                    </el-form>
                  </div>

                  <div class="mb-4">
                    <div class="text-md font-medium mb-2">机械臂（串口）</div>
                    <el-form :model="robotConfig" label-width="120px">
                      <el-form-item label="串口">
                        <el-input v-model="robotConfig.port" />
                      </el-form-item>
                      <el-form-item>
                        <el-button type="primary" size="small" @click="saveRobot">保存机械臂</el-button>
                      </el-form-item>
                    </el-form>
                  </div>
                </div>
              </el-tab-pane>

              <el-tab-pane label="系统集成" name="integration">
                <div class="p-4">
                  <el-form :model="mesConfig" label-width="120px">
                    <el-form-item label="启用 MES">
                      <el-switch v-model="mesConfig.enabled" />
                    </el-form-item>
                    <el-form-item label="MES 地址">
                      <el-input v-model="mesConfig.url" placeholder="http://mes.example/api" />
                    </el-form-item>
                    <el-form-item label="访问令牌">
                      <el-input v-model="mesConfig.token" />
                    </el-form-item>
                    <el-form-item>
                      <el-button type="primary" @click="saveMesConfig">保存 MES 配置</el-button>
                      <el-button type="info" @click="testMesConnection">测试连接</el-button>
                    </el-form-item>
                  </el-form>
                </div>
              </el-tab-pane>
            </el-tabs>
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>

    <!-- User add/edit dialog -->
    <el-dialog :title="userDialogTitle" v-model="userDialogVisible" append-to-body width="560px">
      <el-form :model="userForm" label-width="100px">
        <el-form-item label="工号">
          <el-input v-model="userForm.employeeId" />
        </el-form-item>
        <el-form-item label="姓名">
          <el-input v-model="userForm.name" />
        </el-form-item>
        <el-form-item label="班组">
          <el-input v-model="userForm.team" />
        </el-form-item>
        <el-form-item label="角色">
          <el-select v-model="userForm.role">
            <el-option label="用户" value="用户" />
            <el-option label="管理员" value="管理员" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="userDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveUser">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

interface UserItem {
  employeeId: string;
  name: string;
  team: string;
  role: '用户' | '管理员';
}

const activeTab = ref('users');

// mock current user role (in real app use auth)
const currentUserRole = ref<'用户' | '管理员'>('管理员');

const users = ref<UserItem[]>([
  { employeeId: '1001', name: '张三', team: 'A组', role: '管理员' },
  { employeeId: '1002', name: '李四', team: 'B组', role: '用户' },
  { employeeId: '1003', name: '王五', team: 'A组', role: '用户' },
]);

// pagination
const currentPage = ref(1);
const pageSize = ref(10);
const usersPaginated = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  return users.value.slice(start, start + pageSize.value);
});

function handleSizeChange(size: number) {
  pageSize.value = size;
  currentPage.value = 1;
}

function handlePageChange(page: number) {
  currentPage.value = page;
}

// add / edit user
const userDialogVisible = ref(false);
const userDialogTitle = ref('新增用户');
const editUserIndex = ref<number | null>(null);
const userForm = ref<UserItem>({ employeeId: '', name: '', team: '', role: '用户' });

function openAddUser() {
  userDialogTitle.value = '新增用户';
  editUserIndex.value = null;
  userForm.value = { employeeId: '', name: '', team: '', role: '用户' };
  userDialogVisible.value = true;
}

function openEditUser(row: UserItem, index: number) {
  userDialogTitle.value = '修改用户';
  editUserIndex.value = index + (currentPage.value - 1) * pageSize.value;
  userForm.value = { ...row };
  userDialogVisible.value = true;
}

function saveUser() {
  if (editUserIndex.value === null) {
    users.value.push({ ...userForm.value });
  } else {
    users.value.splice(editUserIndex.value, 1, { ...userForm.value });
  }
  userDialogVisible.value = false;
}

function deleteUser(indexOnPage: number) {
  if (currentUserRole.value !== '管理员') return;
  const realIndex = indexOnPage + (currentPage.value - 1) * pageSize.value;
  users.value.splice(realIndex, 1);
}

function resetPassword(indexOnPage: number) {
  if (currentUserRole.value !== '管理员') return;
  const realIndex = indexOnPage + (currentPage.value - 1) * pageSize.value;
  const u = users.value[realIndex];
  // In real app call API to reset and notify; here just show console
  console.info(`Password for ${u.employeeId} reset to default.`);
  // Optionally show a notification (requires $message or global call)
}

import { reactive } from 'vue';

// --- basic config ---
const systemTab = ref('basic');
const basicConfig = reactive({
  db: { host: '127.0.0.1', port: 5432, user: 'sa', password: '', database: 'easydb' },
  log: { level: 'INFO', maxSize: 50 },
  storage: { path: 'E:/data/storage' },
});

function chooseStorageDir() {
  // placeholder: in a real Tauri/Electron app open native dialog
  console.info('choose storage dir (native dialog not implemented)');
}

function saveBasicConfig() {
  console.info('Save basic config', JSON.parse(JSON.stringify(basicConfig)));
}

// --- hardware ---
const plcConfig = reactive({ ip: '192.168.0.10', port: 502 });
const cameras = ref([{ name: 'Camera A', port: 'COM3' }]);
const lights = ref([{ name: 'Light A', port: 'COM4' }]);
const scannerConfig = reactive({ port: 'COM5' });
const robotConfig = reactive({ port: 'COM6' });

// camera dialog
const cameraDialogVisible = ref(false);
const cameraForm = ref({ name: '', port: '' });
const cameraEditIndex = ref<number | null>(null);
function openAddCamera() { cameraDialogVisible.value = true; cameraEditIndex.value = null; cameraForm.value = { name: '', port: '' }; }
function openEditCamera(row: any, index: number) { cameraDialogVisible.value = true; cameraEditIndex.value = index; cameraForm.value = { ...row }; }
function saveCamera() { if (cameraEditIndex.value === null) cameras.value.push({ ...cameraForm.value }); else cameras.value.splice(cameraEditIndex.value, 1, { ...cameraForm.value }); cameraDialogVisible.value = false; }
function removeCamera(i: number) { cameras.value.splice(i, 1); }

// light dialog
const lightDialogVisible = ref(false);
const lightForm = ref({ name: '', port: '' });
const lightEditIndex = ref<number | null>(null);
function openAddLight() { lightDialogVisible.value = true; lightEditIndex.value = null; lightForm.value = { name: '', port: '' }; }
function openEditLight(row: any, index: number) { lightDialogVisible.value = true; lightEditIndex.value = index; lightForm.value = { ...row }; }
function saveLight() { if (lightEditIndex.value === null) lights.value.push({ ...lightForm.value }); else lights.value.splice(lightEditIndex.value, 1, { ...lightForm.value }); lightDialogVisible.value = false; }
function removeLight(i: number) { lights.value.splice(i, 1); }

function savePlc() { console.info('Save PLC', JSON.parse(JSON.stringify(plcConfig))); }
function saveScanner() { console.info('Save scanner', JSON.parse(JSON.stringify(scannerConfig))); }
function saveRobot() { console.info('Save robot', JSON.parse(JSON.stringify(robotConfig))); }

// --- system integration (MES) ---
const mesConfig = reactive({ enabled: false, url: '', token: '' });
function saveMesConfig() { console.info('Save MES config', JSON.parse(JSON.stringify(mesConfig))); }
function testMesConnection() { console.info('Test MES connection to', mesConfig.url); }

</script>

<style scoped>
.text-lg { font-size: 16px }
.font-medium { font-weight: 500 }
.mb-4 { margin-bottom: 16px }
.gap-4 { gap: 16px }
.flex { display: flex }
.justify-between { justify-content: space-between }
.items-center { align-items: center }
.pagination-wrap { margin-top: 12px }
</style>
