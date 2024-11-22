<template>
   <n-scrollbar style="max-height: 100%">
     <!-- <div style="border-top: 5px solid #3064b1;"></div> -->
    <!-- 头 -->
    <div class="h100 w1000 m-0 m-auto flex flex-row flex-justify-between flex-items-center">
       <div class="flex flex-items-center flex-justify-center flex-row">
            <div class="h80 w80 bg-#444 flex flex-items-center flex-justify-center color-white m-10">
                Logo
            </div>
            <div class="ml-20 flex flex-col flex-items-center">
                <div class="font-size-24 font-bold c-##3064b1 c-#3064b1">XX公司</div>
                <div class="c-##3064b1 font-bold c-#3064b1">xxxx</div>
            </div>
       </div>
       <div class="flex flex-row">
            <div class="flex flex-row flex-items-center m-r-10">
            语言：
            <n-select style="width: 100px;" />
        </div>
            <n-input class=" mr-10" style="width: 200px" placeholder="搜索" >
                    <template #suffix>
                    <div @click="hello" class="i-material-symbols:search"></div>
                    </template>
                </n-input>
       </div>
    </div>
    <!-- 导航页 -->
     <div class="h50 w100% m-0 m-auto bg-#3064b1 flex flex-row flex-justify-around  c-#fff">
        <div class="item">首页</div>
        <div class="item">新闻</div>
        <div class="item">产品介绍</div>
        <div class="item">关于</div>
     </div>
     <!-- banner -->
    <div class="w100% h300 bg-#666 c-#fff flex flex-row flex-justify-center flex-items-center font-size-24">
        滚动图
    </div>

    <div class="w100% h600 bg-#4848 c-#fff flex flex-row flex-justify-center flex-items-center font-size-24">
        介绍的产品内容
    </div>

    <div class="w100% h140 bg-#222222 c-#999 flex flex-row flex-justify-around flex-items-center b-b b-b-solid b-#444">
       <div class="flex flex-row items-center">
        <div class=" w50 h50 bg-#444 m-r-10 "></div>
        <div class=" flex flex-col b-l b-l-solid b-#444 p-l-10">
            <span>联系地址</span>
            <span>xx市xx区xx路xxx号</span>
            <span>xx栋xx楼xxx号</span>
        </div>
       </div>
       <div class="flex flex-row items-center">
        <div class=" w50 h50 bg-#444 m-r-10 "></div>
        <div class=" flex flex-col b-l b-l-solid b-#444 p-l-10">
            <span>联系电话</span>
            <span>13xxxxxxxxxx</span>
            <span>021-xxxxxx</span>
        </div>
       </div>
       <div class="flex flex-row items-center">
        <div class=" w50 h50 bg-#444 m-r-10 "></div>
        <div class=" flex flex-col b-l b-l-solid b-#444 p-l-10">
            <span>联系邮箱</span>
            <span>xxxxxxx@xxxx.com</span>
        </div>
       </div>

    </div>
    <div class="w100% h60 bg-#222222 c-#999 flex flex-row flex-justify-center flex-items-center">
        ©2024 xxx公司 版权所有  ICP备案号:xxxx 
    </div>
</n-scrollbar>


    <!-- <div @click="toLogin">
        去登录
    </div> -->
</template>
<script setup>


  const router = useRouter()
async function toLogin() {
  
    router.push('login')
}

async function hello() {
  
 alert("123")
}

</script>
<style>
.item{
    width: 100px;
    height: 50px;
    display:flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    user-select: none;
}

.item:hover{
   background: #183873;
}

    
</style>