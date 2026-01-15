<template>
  <UContainer class="py-8 space-y-8">
    <DashboardHeader />

    <DashboardStats :stats="stats" />

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <DashboardChartCard />
      <DashboardOrders :orders="recentOrders" />
    </div>

    <DashboardServerStatus/>

  </UContainer>
</template>

<script setup lang="ts">
import DashboardHeader from '@/components/Dashboard/DashboardHeader.vue'
import DashboardStats from '@/components/Dashboard/DashboardStats.vue'
import DashboardChartCard from '@/components/Dashboard/DashboardChartCard.vue'
import DashboardOrders from '@/components/Dashboard/DashboardOrders.vue'
import DashboardServerStatus from '@/components/Dashboard/DashboardServerStatus.vue'


const stats = [
  { label: 'Книг в базе', value: '', delta: '', icon: 'i-lucide-library' },
  { label: 'Активных авторов', value: '', delta: '', icon: 'i-lucide-users' },
  { label: 'Выдано сегодня', value: '', delta: '', icon: 'i-lucide-book-up' },
]

const recentOrders = [
  { id: '', book: '', user: '', status: '' }
]
</script>
