<template>
  <div class="flex flex-row w-full">
    <div class="flex flex-col items-start justify-center">
      <div class="text-size-25px font-bold">
        {{ productionLineConfig.productionLineName }}
      </div>
      <div class="text-size-16px text-#696969 leading-34px">
        {{ productionLineConfig.workstationName }}
      </div>
    </div>
    <div class=" flex flex-row flex-1 align-center justify-center  gap-20px font-600 ">
      <div  class=" flex flex-col align-center w-150px  ">
        <div class="text-#606060" > 检测数量</div>
        <div class="text-size-30px leading-35px">231</div>
      </div>
      <div  class=" flex flex-col align-center w-150px ">
        <div class="text-#606060">合格数量</div>
        <div class="text-size-30px leading-35px">229</div>
      </div>
      <div  class=" flex flex-col align-center w-150px ">
        <div class="text-#606060">不合格数量</div>
        <div class="text-size-30px leading-35px">2</div>
      </div>  
      <div  class=" flex flex-col align-center w-150px ">
        <div class="text-#606060">合格率</div>
        <div class="text-size-30px leading-35px">99.9%</div>
      </div>
    </div>
    <div class =" flex flex-row  gap-10px font-600 ">
      <div class="flex flex-col items-center select-none cursor-pointer w-60px active:bg-#afafaf">
        <div class="w-40px h-40px rounded-8px bg-#F7E2E2 flex items-center justify-center mt-5px">
          <img src="../assets/Stop.svg" alt="" class="w-25px h-25px">
        </div>
        <div class="font-400 text-14px" >停止</div>
      </div>
    </div>
    <div class="flex flex-col items-start justify-center ml-20px">
      <div class="flex flex-row flex-1 items-center">
        <div class="text-14px"> 运行状态:</div>
        <div class=" flex flex-row items-center text-12px bg-#0486FEA0 bg-opacity-14 rounded-4px text-#00A33C pl-5px pr-5px ml-10px">
          <div class="w-10px h-10px rounded-50% bg-#00A33C mr-5px" ></div>  
          <div>自动检测中</div>
        </div>
      </div>
      <div class="flex flex-row flex-1 items-center">
        <div class="text-14px"> 运行时长:</div>
        <div class="ml-10px text-14px"> 01:23:45</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getConfig, ProductionLineConfig } from '../config/productionLine';
const productionLineConfig = ref<ProductionLineConfig>(getConfig());
onMounted(() => {
  console.log('Production Line Config:', productionLineConfig);
});
</script>