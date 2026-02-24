<template>
  <div class="user-management flex flex-col gap-4">
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