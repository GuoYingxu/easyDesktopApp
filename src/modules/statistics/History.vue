<template>
  <div class="history flex flex-col h-full max-h-full overflow-hidden">
    <div class="max-h-full h-full overflow-hidden bg-white p-16px rounded-8px">
      <el-tabs v-model="activeTab" class="overflow-hidden">
         <el-tab-pane label="数据统计" name="stats">
          <div class="stats-controls">
            <el-form :inline="true">
              <el-form-item label="时间范围">
                <el-date-picker
                  v-model="statsRange"
                  type="daterange"
                  range-separator="至"
                  start-placeholder="开始日期"
                  end-placeholder="结束日期"
                  value-format="yyyy-MM-dd"
                />
              </el-form-item>
              <el-form-item>
                <el-button type="primary" @click="updateCharts">刷新统计</el-button>
              </el-form-item>
            </el-form>
          </div>

          <div class="charts-grid  w-full gap-16px" style='height: calc(100vh - 300px)'>
            <div ref="chartProd" class="w-full h-full" />
            <div class="flex flex-col flex-40%">
              <div ref="chartModel" class="w-full h-full " />
              <div ref="chartDefect" class="w-full h-full " />
            </div>
          </div>
        </el-tab-pane>
        <el-tab-pane label="历史数据查询" name="query">
          <div class="filter-row">
            <el-form :inline="true" class="filter-form">
              <el-form-item label="时间范围">
                <el-date-picker
                  v-model="filters.dateRange"
                  type="daterange"
                  range-separator="至"
                  start-placeholder="开始日期"
                  end-placeholder="结束日期"
                  value-format="yyyy-MM-dd HH:mm:ss"
                />
              </el-form-item>
              <el-form-item label="型号">
                <el-input v-model="filters.model" placeholder="输入产品型号" clearable />
              </el-form-item>
              <el-form-item label="检测结果">
                <el-select v-model="filters.result" placeholder="全部" clearable  style="width: 240px">
                  <el-option label="全部" value="" />
                  <el-option label="合格" value="合格" />
                  <el-option label="不合格" value="不合格" />
                </el-select>
              </el-form-item>
              <el-form-item>
                <el-button type="primary" @click="applyFilters">查询</el-button>
                <el-button @click="resetFilters">重置</el-button>
              </el-form-item>
            </el-form>
          </div>

          <el-table :data="paginatedRecords" stripe style="width: 100%; height: calc( 100vh - 400px);" >
            <el-table-column label="序号" type="index" width="80" >
            </el-table-column>
            <el-table-column prop="time" label="时间" width="200" />
            <el-table-column prop="model" label="产品型号"  />
            <el-table-column prop="serial" label="序列号" />
            <el-table-column prop="result" label="检测结果" width="120">
              <template #default="{ row }">
                <el-tag :type="row.result === '合格' ? 'success' : 'danger'">{{ row.result }}</el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="defects" label="缺陷" />
            <el-table-column label="操作" align="center" >
              <template #default="{ row }">
                <el-button text type="primary" >详情</el-button>
              </template>
              </el-table-column>
          </el-table>
          <div class="pagination-wrap">
            <el-pagination
              background
              layout="total, sizes, prev, pager, next, jumper"
              :total="filteredTotal"
              :page-size="pageSize"
              :current-page.sync="currentPage"
              :page-sizes="[15,30,50]"
              @size-change="handleSizeChange"
              @current-change="handlePageChange"
            />
          </div>
        </el-tab-pane>

       
      </el-tabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, watch, nextTick } from 'vue';
import * as echarts from 'echarts';

interface RecordItem {
  time: string; // ISO or formatted
  model: string;
  serial: string;
  result: ''|'合格' | '不合格';
  defects: string; // comma separated or description
}

// --- sample/mock data ---
function generateMockData(): RecordItem[] {
  const models = ['A100', 'B200', 'C300'];
  const defectsPool = ['划痕', '凹陷', '掉漆', '异物', 'LOGO'];
  const items: RecordItem[] = [];
  const now = Date.now();
  for (let i = 0; i < 200; i++) {
    const t = new Date(now - Math.floor(Math.random() * 1000 * 60 * 60 * 24 * 30));
    const model = models[Math.floor(Math.random() * models.length)];
    const isOk = Math.random() > 0.25;
    const defects = isOk ? '' : defectsPool[Math.floor(Math.random() * defectsPool.length)];
    items.push({
      time: t.toISOString().replace('T', ' ').slice(0, 19),
      model,
      serial: `SN${Math.floor(100000 + Math.random() * 900000)}`,
      result: isOk ? '合格' : '不合格',
      defects,
    });
  }
  // sort desc
  return items.sort((a, b) => (a.time < b.time ? 1 : -1));
}

const activeTab = ref('stats');
const allRecords = ref<RecordItem[]>(generateMockData());

// Filters for query tab
const filters = ref({ dateRange: [] as string[] | null, model: '', result: '' });

// pagination
const currentPage = ref(1);
const pageSize = ref(15);

const filteredRecords = computed(() => {
  const list = allRecords.value.filter((r) => {
    if (filters.value.model && !r.model.includes(filters.value.model)) return false;
    if (filters.value.result && r.result !== filters.value.result) return false;
    if (filters.value.dateRange && filters.value.dateRange.length === 2) {
      const [start, end] = filters.value.dateRange;
      const t = r.time;
      if (start && end) {
        if (t < start || t > end) return false;
      }
    }
    return true;
  });
  return list;
});

const filteredTotal = computed(() => filteredRecords.value.length);

const paginatedRecords = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  return filteredRecords.value.slice(start, start + pageSize.value);
});

function applyFilters() {
  currentPage.value = 1;
}

function resetFilters() {
  filters.value = { dateRange: [], model: '', result: '' };
  currentPage.value = 1;
}

function handleSizeChange(size: number) {
  pageSize.value = size;
  currentPage.value = 1;
}

function handlePageChange(page: number) {
  currentPage.value = page;
}

// --- Statistics tab ---
const statsRange = ref<string[] | null>(null);
const chartProd = ref<HTMLDivElement | null>(null);
const chartModel = ref<HTMLDivElement | null>(null);
const chartDefect = ref<HTMLDivElement | null>(null);
let prodChart: echarts.ECharts | null = null;
let modelChart: echarts.ECharts | null = null;
let defectChart: echarts.ECharts | null = null;

function computeStats(records: RecordItem[]) {
  // Production over days (count total and qualified)
  const dayMap = new Map<string, { total: number; ok: number }>();
  records.forEach((r) => {
    const day = r.time.slice(0, 10);
    const v = dayMap.get(day) || { total: 0, ok: 0 };
    v.total++;
    if (r.result === '合格') v.ok++;
    dayMap.set(day, v);
  });

  const days = Array.from(dayMap.keys()).sort();
  const totalSeries = days.map((d) => dayMap.get(d)!.total);
  const okSeries = days.map((d) => dayMap.get(d)!.ok);

  // Model distribution
  const modelMap = new Map<string, number>();
  records.forEach((r) => modelMap.set(r.model, (modelMap.get(r.model) || 0) + 1));
  const modelData = Array.from(modelMap.entries()).map(([name, value]) => ({ name, value }));

  // Defect distribution
  const defectMap = new Map<string, number>();
  records.forEach((r) => {
    if (r.defects) {
      const parts = r.defects.split(',').map((s) => s.trim()).filter(Boolean);
      parts.forEach((p) => defectMap.set(p, (defectMap.get(p) || 0) + 1));
    }
  });
  const defectData = Array.from(defectMap.entries()).map(([name, value]) => ({ name, value }));

  return { days, totalSeries, okSeries, modelData, defectData };
}

function initCharts() {
  if (chartProd.value) prodChart = echarts.init(chartProd.value);
  if (chartModel.value) modelChart = echarts.init(chartModel.value);
  if (chartDefect.value) defectChart = echarts.init(chartDefect.value);
  updateCharts();
}

function updateCharts() {
  // filter by statsRange if set
  let records = allRecords.value;
  if (statsRange.value && statsRange.value.length === 2) {
    const [start, end] = statsRange.value;
    const s = start + 'T00:00:00';
    const e = end + 'T23:59:59';
    records = records.filter((r) => r.time >= s && r.time <= e);
  }

  const { days, totalSeries, okSeries, modelData, defectData } = computeStats(records);

  if (prodChart) {
    prodChart.setOption({
      title: { text: '产量统计（按日）' },
      tooltip: { trigger: 'axis' },
      legend: { data: ['总产量', '合格'] },
      xAxis: { type: 'category', data: days },
      yAxis: { type: 'value' },
      series: [
        { name: '总产量', type: 'bar', data: totalSeries },
        { name: '合格', type: 'line', data: okSeries },
      ],
    });
  }

  if (modelChart) {
    modelChart.setOption({
      title: { text: '型号分布' },
      tooltip: { trigger: 'item' },
      legend: { orient: 'vertical', left: 'left' },
      series: [{ name: '型号', type: 'pie', radius: '50%', data: modelData }],
    });
  }

  if (defectChart) {
    defectChart.setOption({
      title: { text: '缺陷分布' },
      tooltip: { trigger: 'item' },
      legend: { orient: 'vertical', left: 'left' },
      series: [{ name: '缺陷', type: 'pie', radius: '50%', data: defectData }],
    });
  }

}

onMounted(() => {
  initCharts();
  window.addEventListener('resize', () => {
    prodChart?.resize();
    modelChart?.resize();
    defectChart?.resize();
  });
  setTimeout(() => {
    prodChart?.resize();
    modelChart?.resize();
    defectChart?.resize();
    console.log('resize');
  },1000);  
});

onBeforeUnmount(() => {
  prodChart?.dispose();
  modelChart?.dispose();
  defectChart?.dispose();
});

watch([() => filters.value, () => pageSize.value], () => {
  currentPage.value = 1;
});


</script>

<style scoped>
.view { padding: 16px }
.filter-row { margin-bottom: 12px; }
.filter-form { width: 100%; align-items: center; }
.pagination-wrap { margin-top: 12px; display:flex; justify-content:flex-end; }
.stats-controls { margin-bottom: 12px; }
.charts-grid { display: grid; grid-template-columns: 3fr 1fr; gap: 12px; }
</style>
