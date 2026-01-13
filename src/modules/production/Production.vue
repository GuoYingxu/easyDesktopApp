<template>
  <div class="flex flex-col h-full">
  <div class ="flex flex-col bg-white p-16px rounded-8px flex-1 overflow-hidden">
      <div class="flex justify-between items-center mb-4">
        <div class="text-lg font-medium">产品管理</div>
        <el-button type="primary" size="small" @click="openAdd">新增产品</el-button>
      </div>

      <el-table :data="paginatedData" stripe style="width: 100%">
        <el-table-column type="index" label="序号" width="80" />
        <el-table-column prop="modelCode" label="型号编码" width="160" />
        <el-table-column prop="spec" label="规格型号" width="160" />
        <el-table-column prop="brand" label="品牌名称" width="140" />
        <el-table-column prop="color" label="颜色" width="100" />
        <el-table-column prop="material" label="材质" width="120" />
        <el-table-column prop="size" label="尺寸" width="120" />
        <el-table-column label="操作" width="160">
          <template #default="{ row, $index }">
            <el-button size="small" @click="openEdit(row, $index)">编辑</el-button>
            <el-button size="small" type="danger" @click="deleteRow($index)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
      <div class="pagination-wrap mt-4" style="display:flex;justify-content:flex-end;margin-top:12px;padding-top:8px;">
        <el-pagination
          background
          layout="total, sizes, prev, pager, next, jumper"
          :total="tableData.length"
          :page-size="pageSize"
          :current-page.sync="currentPage"
          :page-sizes="[5,10,20,50]"
          @size-change="handleSizeChange"
          @current-change="handlePageChange"
        />
      </div>
    </div>

    <el-dialog :title="dialogTitle" v-model="dialogVisible" append-to-body width="640px">
      <el-form :model="form" label-width="100px">
        <el-form-item label="型号编码">
          <el-input v-model="form.modelCode" />
        </el-form-item>
        <el-form-item label="规格型号">
          <el-input v-model="form.spec" />
        </el-form-item>
        <el-form-item label="品牌名称">
          <el-input v-model="form.brand" />
        </el-form-item>
        <el-form-item label="颜色">
          <el-input v-model="form.color" />
        </el-form-item>
        <el-form-item label="材质">
          <el-input v-model="form.material" />
        </el-form-item>
        <el-form-item label="尺寸">
          <el-input v-model="form.size" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveRow">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

interface Product {
  modelCode: string;
  spec: string;
  brand: string;
  color: string;
  material: string;
  size: string;
}

const tableData = ref<Product[]>([
  { modelCode: 'P-100', spec: '100A', brand: 'BrandX', color: '白色', material: '铝', size: '100x50' },
  { modelCode: 'P-200', spec: '200B', brand: 'BrandY', color: '黑色', material: '塑料', size: '120x60' },
]);

const dialogVisible = ref(false);
const dialogTitle = ref('新增产品');
const editIndex = ref<number | null>(null);
const form = ref<Product>({ modelCode: '', spec: '', brand: '', color: '', material: '', size: '' });

// pagination
const currentPage = ref(1);
const pageSize = ref(10);

const paginatedData = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  return tableData.value.slice(start, start + pageSize.value);
});

function handleSizeChange(size: number) {
  pageSize.value = size;
  currentPage.value = 1;
}

function handlePageChange(page: number) {
  currentPage.value = page;
}

function openAdd() {
  dialogTitle.value = '新增产品';
  editIndex.value = null;
  form.value = { modelCode: '', spec: '', brand: '', color: '', material: '', size: '' };
  dialogVisible.value = true;
}

function openEdit(row: Product, index: number) {
  dialogTitle.value = '编辑产品';
  editIndex.value = index;
  form.value = { ...row };
  dialogVisible.value = true;
}

function saveRow() {
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
</script>

<style scoped>
.justify-between { justify-content: space-between }
.items-center { align-items: center }
.text-lg { font-size: 16px }
.font-medium { font-weight: 500 }
</style>
