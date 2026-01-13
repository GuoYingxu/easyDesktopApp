<template>
  <div class="padding-16px flex flex-row h-full">
    <div class="bg-white flex-1 flex flex-col rounded-8px mr-16px p-16px border-solid border-2px border-#0486FE">
      <div class="flex flex-row gap-12">
        <div class="font-bold"> 实时检测</div>
        <div class="flex flex-row">
          <div class="font-bold"> 工单号: </div>
          <div class="bg-#f0f0f0 pl-8px pr-8px ml-8px"> 123456880384984</div>
        </div>
        <div class="flex flex-row">
          <div class="font-bold"> 产品型号: </div>
          <div class="bg-#f0f0f0 pl-8px pr-8px ml-8px"> 123456880384984</div>
        </div>
      </div>
      <div class="flex-1 flex flex-row mt-8px gap-20px">
        <div class="bg-black flex-1 flex flex-col items-center w-full h-full">
          <img src="../../assets/bx.jpg" class="object-contain  h-700px max-h-full max-w-full">
        </div>
        <div class="w-250px flex flex-col gap-20px" >
          <div class="w-full  bg-#0486FE bg-opacity-9 rounded-8px p-16px box-border border-solid border-2px border-#BB0A0A">
            <div class="flex flex-row font-600 text-25px">
              <div >检测结果:</div>
              <div class="ml-10px text-#BB0A0A"> 不合格 </div>
            </div>
            <div class="flex flex-row text-#606060 mt-25px leading-25px">
              <div> 检测耗时：</div>
              <div > 1500ms</div>
            </div>
            <div class="flex flex-row text-#606060 leading-25px">
              <div> 置信度：</div>
              <div > 98.9%</div>
            </div>
          </div>
          <div class="w-full bg-#F4F3F3 bg-opacity-66 flex-1 rounded-8px box-border p-16px" >
            <div class="font-bold">检测项目结果</div>
            <div class="mt-20px flex flex-col gap-10px">
              <div class="flex flex-row ">
                <img src="../../assets/error.svg" class="mr-10px w-20px h-20px">
                <div> 划痕 <span class="ml-50px">1</span></div>
              </div>
              <div class="flex flex-row  ">
                <img src="../../assets/Success.svg" class="mr-10px w-20px h-20px">
                <div> 凹陷 </div>
              </div>
                 <div class="flex flex-row   ">
                <img src="../../assets/Success.svg" class="mr-10px w-20px h-20px">
                <div> 掉漆 </div>
              </div>
               <div class="flex flex-row  ">
                <img src="../../assets/Success.svg" class="mr-10px w-20px h-20px ">
                <div> 异物 </div>
              </div>
               <div class="flex flex-row   ">
                <img src="../../assets/Success.svg" class="mr-10px w-20px h-20px">
                <div> LOGO </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="w-450px flex flex-col gap-16px">
      <div class="flex flex-col gap-16px">
        <div class="flex flex-row gap-16px">
          <div class="border-solid border-2px border-#0486FE bg-white flex-1 flex flex-row h-150px justify-center items-center gap-12px p-16px box-border rounded-8px">
            <div class="w-70px h-70px bg-#21C600 bg-opacity-19 rounded-50% flex items-center justify-center justify-center" >
              <img src="../../assets/camera.svg" class=" w-44px h-44px">
            </div>
            <div class="flex flex-col"> 
              <div class="text-20px font-bold leading-40px"> 相机组</div>
              <div class="text-18px leading-40px text-#16A34A"> 在线 （1/1）</div>
            </div>
          </div>
          <div class= " border-solid border-2px border-#0486FE bg-white flex-1 flex flex-row h-150px justify-center items-center gap-12px p-16px box-border rounded-8px">
            <div class="w-70px h-70px bg-#21C600 bg-opacity-19 rounded-50% flex items-center justify-center justify-center" >
              <img src="../../assets/light.svg" class=" w-44px h-44px">
            </div>
            <div class="flex flex-col"> 
              <div class="text-20px font-bold leading-40px"> 光源控制器</div>
              <div class="text-18px leading-40px text-#16A34A"> 正常 </div>
            </div>
          </div>
        </div>
         <div class="flex flex-row gap-16px">
          <div class=" border-solid border-2px border-#0486FE bg-white flex-1 flex flex-row h-150px justify-center items-center gap-12px p-16px box-border rounded-8px">
            <div class="w-70px h-70px bg-#0D06FF bg-opacity-19 rounded-50% flex items-center justify-center justify-center" >
              <img src="../../assets/Chip.svg" class=" w-44px h-44px">
            </div>
            <div class="flex flex-col"> 
              <div class="text-20px font-bold leading-40px"> PLC 通信</div>
              <div class="text-18px leading-40px text-#16A34A"> 12ms</div>
            </div>
          </div>
          <div class=" border-solid border-2px border-#0486FE bg-white flex-1 flex flex-row h-150px justify-center items-center gap-12px p-16px box-border rounded-8px">
            <div class="w-70px h-70px bg-#0486FE bg-opacity-19 rounded-50% flex items-center justify-center justify-center" >
              <img src="../../assets/temp-hot-line.svg" class=" w-44px h-44px">
            </div>
            <div class="flex flex-col "> 
              <div class="text-20px font-bold leading-40px"> 机柜温度</div>
              <div class="text-18px leading-40px text-#16A34A"> 38 ℃ </div>
            </div>
          </div>
        </div>
        
      </div>
      <div class="flex-1  bg-white border-solid border-2px border-#0486FE rounded-8px p-16px overflow-hidden">
        <div class="flex flex-row  justify-between">
          <div class="font-bold"> 检测记录</div>
          <el-button > 查看详情   </el-button>
        </div>
        <div class="mt-10px"> 
          <div v-for="item in record" :key="item.id" class="flex flex-row justify-between border-b-1px border-#e6e6e6 pb-8px pt-8px">
            <div class="flex flex-row gap-12px items-center">
              <img v-if='item.result === "不合格"' src="../../assets/error.svg" class="w-20px h-20px" >
              <img v-else src="../../assets/Success.svg" class="w-20px h-20px" >
              <div class="text-14px"> {{ item.time }} </div>
              <div> {{ item.id }} </div>
            </div>
            <div>
              {{ item.result }}
            </div>
          </div>
        </div>

       </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue';
  import { ElButton } from 'element-plus';
  const record= ref([
    { time: '2021-09-01 12:00:00', id: 'SN124399203', result: '不合格', },
    { time: '2021-09-01 12:00:00', id: 'SN124399204', result: '合格', },
     { time: '2021-09-01 12:00:00', id: 'SN124399204', result: '合格', },
      { time: '2021-09-01 12:00:00', id: 'SN124399204', result: '合格', },
       { time: '2021-09-01 12:00:00', id: 'SN124399204', result: '合格', },
        { time: '2021-09-01 12:00:00', id: 'SN124399204', result: '合格', }, { time: '2021-09-01 12:00:00', id: 'SN124399204', result: '合格', }, { time: '2021-09-01 12:00:00', id: 'SN124399204', result: '合格', },
        
         { time: '2021-09-01 12:00:00', id: 'SN124399204', result: '合格', },
          { time: '2021-09-01 12:00:00', id: 'SN124399204', result: '合格', },
           { time: '2021-09-01 12:00:00', id: 'SN124399204', result: '合格', },



    ])
</script>

<style scoped>
.view { padding: 16px; }
</style>
