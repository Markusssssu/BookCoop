<template>
  <UCard class="lg:col-span-2">
    <template #header>
      <div class="flex items-center justify-between">
        <h3 class="font-bold">Активность читателей</h3>
        <USelectMenu
            :options="['7 дней', '30 дней', 'Год']"
            size="sm"
            class="w-32"
        />
      </div>
    </template>

    <DashboardChart />
  </UCard>
</template>
