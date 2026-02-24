<template>
  <div class="config  flex flex-col h-full max-h-full overflow-hidden">
    <div class="max-h-full h-full overflow-hidden bg-white p-16px rounded-8px">
      <el-tabs v-model="activeTab">
        <el-tab-pane label="检测设置" name="settings">
          <div class="flex flex-col gap-4">
            <div class="flex justify-end">
              <el-button type="primary" size="small" @click="openAdd">新增配置</el-button>
            </div>

            <el-table :data="tableData" stripe style="width: 100%">
              <el-table-column type="index" label="序号" width="80" />
              <el-table-column prop="modelCode" label="型号编码" width="160" />
              <el-table-column prop="color" label="颜色" width="120" />
              <el-table-column prop="material" label="材质" width="120" />
              <el-table-column prop="setting" label="检测设置" />
              <el-table-column prop="model" label="模型" width="160" />
              <el-table-column prop="rule" label="检测规则" />
              <el-table-column label="操作" width="160">
                <template #default="{ row, $index }">
                  <el-button size="small" @click="openEdit(row, $index)">编辑</el-button>
                  <el-button size="small" type="danger" @click="deleteRow($index)">删除</el-button>
                </template>
              </el-table-column>
            </el-table>
          </div>
        </el-tab-pane>

        <el-tab-pane label="检测规则" name="rules">
          <div class="flex flex-col gap-4">
            <div class="flex justify-between items-center">
              <div class="text-lg font-medium">规则列表</div>
              <el-button type="primary" size="small" @click="openAddRule">新增规则</el-button>
            </div>
              <el-table :data="rules" stripe style="width:100%">
              <el-table-column type="index" label="序号" width="80" />
              <el-table-column prop="name" label="规则名称" />
              <el-table-column prop="model" label="选用模型" />
              <el-table-column prop="huahen" label="划痕" >
                <template #default="scope">
                  <el-checkbox v-model="scope.row.huahen" :label="scope.row.settings.huahen" />
                  
                </template>
              </el-table-column>
              <el-table-column prop= "aoxian" label="凹陷" >
                 <template #default="scope">
                  <el-checkbox v-model="scope.row.aoxian" />
                </template>
              </el-table-column>
              <el-table-column prop="yiwu" label="异物" >
                <template #default="scope">
                  <el-checkbox v-model="scope.row.yiwu" />
                </template>
              </el-table-column>
              <el-table-column prop="logo" label="LOGO" >
                <template #default="scope">
                  <el-checkbox v-model="scope.row.logo" />
                </template>
              </el-table-column>
              <el-table-column prop="description" label="报警设置" />
              <el-table-column label="操作" width="160">
                <template #default="{ $index }">
                  <el-button size="small" @click="editRule($index)">编辑</el-button>
                  <el-button size="small" type="danger" @click="deleteRule($index)">删除</el-button>
                </template>
              </el-table-column>
            </el-table>
          </div>
        </el-tab-pane>

        <el-tab-pane label="模型管理" name="models">
          <div class="flex flex-col gap-4">
            <div class="flex justify-between items-center">
              <div class="text-lg font-medium">模型列表</div>
              <div>
                <el-upload action="#" :show-file-list="false" :before-upload="beforeUpload">
                  <el-button size="small" type="primary">上传模型</el-button>
                </el-upload>
              </div>
            </div>
            <el-table :data="models" stripe style="width:100%">
              <el-table-column type="index" label="序号" width="80" />
              <el-table-column prop="name" label="模型名称" width="160" />
              <el-table-column prop="version" label="版本" width="120" />
              <el-table-column prop="description" label="描述" />
              <el-table-column prop="updateAt" label="更新时间" width="160"></el-table-column>
              <el-table-column label="操作" width="160">
                <template #default="{ row, $index }">
                  <el-button size="small" type="danger" @click="removeModel($index)">删除</el-button>
                </template>
              </el-table-column>
            </el-table>
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>

  <el-dialog :title="dialogTitle" v-model="dialogVisible" append-to-body width="640px">
    <div class="w-full p-16px box-border">
      <div class="flex flex-row w-full">
        <div class="leading-35px"> 型号编码: </div>
        <div class='ml-10px flex-1'>
          <el-input v-model="form.modelCode" />
          <div v-if="form.modelCode" class="w-full h-50px mt-10px bg-#C5D0C5 bg-opacity-38 rounded-10px p-10px">白色、 钣金</div>
        </div>
      </div>

      <div class="flex flex-col w-full mt-10px">
        <div class="leading-35px"> 流程设置: </div>
        <div class='ml-10px flex-1'>
          <el-timeline style="max-width: 600px">
            <el-timeline-item v-for="(activity, index) in activities" :key="index" >
              <div class="flex flex-col">
                <div class="flex flex-row justify-start items-center">
                  {{ activity.content }} 
                  <el-checkbox v-model="activity.enabled" label="" class="ml-10px"> </el-checkbox>
                </div>
                <div v-if="index==1 && activity.enabled" class="ml-20px">
                  <div class="flex flex-row justify-start items-center">
                    <div>预处理:</div>
                    <el-button text type="primary" class="ml-10px" size="mini">上传JSON</el-button>
                    <el-button text type="primary" class="ml-10px" size="mini">绘制</el-button>
                  </div>
                  <div class="flex flex-row gap-10px justify-center items-center">
                    <div >
                      选择检测规则:
                    </div> 
                    <el-select v-model="form.rule" placeholder="选择规则" class="flex-1">
                      <el-option
                        v-for="rule in rules"
                        :key="rule.name"
                        :label="rule.name"
                        :value="rule.name"
                      />
                      </el-select>
                  </div>
                    <div class="w-full h-50px mt-10px bg-#C5D0C5 bg-opacity-38 rounded-10px p-8px">{{ selectedRule?.description }}</div>
                </div>
              </div>
            </el-timeline-item>
          </el-timeline>
        </div>
      </div>
    </div>
    <template #footer>
      <el-button @click="dialogVisible = false">取消</el-button>
      <el-button type="primary" @click="saveRow">保存</el-button>
    </template>
  </el-dialog>
  
  <el-dialog :title="ruleDialogTitle" v-model="ruleDialogVisible" append-to-body width="560px">
    <el-form :model="ruleForm" label-width="100px">
      <el-form-item label="规则名称">
        <el-input v-model="ruleForm.name" />
      </el-form-item>
      <el-form-item label="选用模型">
        <el-select v-model="ruleForm.model" placeholder="选择模型">
          <el-option v-for="m in models" :key="m.name" :label="m.name" :value="m.name" />
        </el-select>
        <div class="w-full h-50px mt-10px bg-#C5D0C5 bg-opacity-38 rounded-10px p-8px">{{ selectedModel?.description }}</div> 
      </el-form-item>
      <el-form-item label="缺陷选择">
        <div class="flex flex-col gap-2">
          <div class="flex flex-row justify-start items-center">
            <el-checkbox v-model="ruleForm.huahen">划痕</el-checkbox>
            <div class="flex flex-row justify-start items-center ml-20px">
              <div> 长 <  </div> <el-input size="small" v-model="huahenSettings['height']" class="w-50px ml-5px mr-5px" > </el-input> <div> mm, </div>
              <div> 宽 <  </div> <el-input size="small" v-model="huahenSettings['width']" class="w-50px ml-5px mr-5px" > </el-input> <div> mm </div>
            </div> 
          </div>
          <el-checkbox v-model="ruleForm.aoxian">凹陷</el-checkbox>
          <el-checkbox v-model="ruleForm.yiwu">异物</el-checkbox>
          <el-checkbox v-model="ruleForm.logo">LOGO</el-checkbox>
        </div>
      </el-form-item>
     <el-form-item label="报警设置">
        <div class="flex flex-row gap-2">
          <el-checkbox v-model="alarm.light">灯光</el-checkbox>
          <el-checkbox v-model="alarm.jingbao">警报</el-checkbox>
          <el-checkbox v-model="alarm.yuyin">语音提醒</el-checkbox>
        </div>
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="ruleDialogVisible = false">取消</el-button>
      <el-button type="primary" @click="saveRule">保存</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { ElDialog, ElTable } from 'element-plus';
const alarm = ref({
  light: false,
  jingbao: false,
  yuyin: false
})
interface SettingRow {
  modelCode: string;
  color: string;
  material: string;
  setting: string;
  model: string;
  rule: string;
}
const huahenSettings =ref({
  height: '',
  width: ''
})
const activeTab = ref('settings');

const tableData = ref<SettingRow[]>([
  { modelCode: 'A100', color: '白色', material: '钣金', setting: '采集；检测', model: 'detec_model_v1', rule: '划痕: 长>1mm' },
  { modelCode: 'B200', color: '灰色', material: '玻璃', setting: '采集；不检测', model: 'detec_m_v2', rule: '无' },
  { modelCode: 'B200', color: '白色', material: 'PCM彩涂板', setting: '不采集;不检测', model: 'detec_m_v2', rule: '无' },
]);

const selectedRule = computed(() => {
  return rules.value.find(r => r.name === form.value.rule);
});

const selectedModel = computed(() => {
  return models.value.find(m => m.name === ruleForm.value.model);
});

// dialog / form for add/edit
const dialogVisible = ref(false);
const dialogTitle = ref('检测配置');
const editIndex = ref<number | null>(null);
const formRef = ref<any>(null);
const form = ref<SettingRow>({ modelCode: '', color: '', material: '', setting: '', model: '', rule: '' });
const activities = ref([
  {
    content: '图像采集',
    enabled: false,
  },
  {
    content: '缺陷检测',
    enabled: false,
  },
])
function openAdd() {
  dialogTitle.value = '新增检测配置';
  editIndex.value = null;
  form.value = { modelCode: '', color: '', material: '', setting: '', model: '', rule: '' };
  dialogVisible.value = true;
}

function openEdit(row: SettingRow, index: number) {
  dialogTitle.value = '编辑配置';
  editIndex.value = index;
  form.value = { ...row };
  dialogVisible.value = true;
}

function saveRow() {
  const fr = formRef.value as any;
  // simple save without validation rules for now
  if (editIndex.value === null) {
    tableData.value.push({ ...form.value });
  } else {
    tableData.value.splice(editIndex.value, 1, { ...form.value });
  }
  dialogVisible.value = false;
}

function deleteRow(index: number) {
  tableData.value.splice(index, 1);
}

// rules
interface RuleItem {
  name: string;
  model?: string;
  huahen?: boolean;
  aoxian?: boolean;
  yiwu?: boolean;
  logo?: boolean;
  settings?: Record<string, string>;
  description?: string;
}

const rules = ref<RuleItem[]>([
  { name: 'rule1', model: 'detec_model_v1', huahen: true, aoxian: false, yiwu: true, logo: false, settings: { huahen: '长<1mm,宽<0.5mm' }, description: '语音播报' },
  { name: 'rule2', model: 'detec_model_v2', huahen: false, aoxian: true, yiwu: false, logo: true, settings: { huahen: '长<1mm,宽<0.5mm' }, description: '播报' },
]);

// rule dialog state
const ruleDialogVisible = ref(false);
const ruleDialogTitle = ref('新增规则');
const ruleEditIndex = ref<number | null>(null);
const ruleForm = ref<RuleItem>({ name: '', settings:{}, model: undefined, huahen: false, aoxian: false, yiwu: false, logo: false,  description: '' });

function openAddRule() {
  ruleDialogTitle.value = '新增规则';
  ruleEditIndex.value = null;
  ruleForm.value = { name: '', model: undefined, huahen: false, aoxian: false, yiwu: false, logo: false, settings: {}, description: '' };
  ruleDialogVisible.value = true;
}

function editRule(i: number) {
  ruleDialogTitle.value = '编辑规则';
  ruleEditIndex.value = i;
  ruleForm.value = { ...rules.value[i] };
  ruleDialogVisible.value = true;
}

function saveRule() {
  if (ruleEditIndex.value === null) {
    rules.value.push({ ...ruleForm.value });
  } else {
    rules.value.splice(ruleEditIndex.value, 1, { ...ruleForm.value });
  }
  ruleDialogVisible.value = false;
}

function deleteRule(i: number) { rules.value.splice(i, 1); }

// models
const models = ref([{ name: 'm_v1', version: '1.0', description: "模型描述1", updateAt: "2023-01-01" }, { name: 'm_v2', version: '2.0', description: "模型描述2", updateAt: "2023-01-02" }]);
function beforeUpload() { return false; }
function useModel(row: any) { /* set as active model */ }
function removeModel(i: number) { models.value.splice(i, 1); }
</script>

<style scoped>
.view {
  padding: 16px
}

.gap-4 {
  gap: 16px
}

.p-4 {
  padding: 16px
}

.flex {
  display: flex
}

.flex-col {
  flex-direction: column
}

.justify-end {
  justify-content: flex-end
}

.justify-between {
  justify-content: space-between
}

.items-center {
  align-items: center
}

.text-lg {
  font-size: 16px
}

.font-medium {
  font-weight: 500
}

.border-b {
  border-bottom: 1px solid #eee
}
</style>
