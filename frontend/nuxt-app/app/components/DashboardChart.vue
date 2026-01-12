<template>
  <div class="h-[300px]">
    <Line :data="chartData" :options="chartOptions" />
  </div>
</template>


<script setup lang="ts">
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  BarElement,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Filler
} from 'chart.js'
import { Line } from 'vue-chartjs'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler
)

const chartData = {
  labels: ['Пн', 'Вт', 'Ср', 'Чт', 'Пт', 'Сб', 'Вс'],
  datasets: [
    {
      label: 'Активность библиотеки',
      backgroundColor: 'rgba(59, 130, 246, 0.1)', // Цвет primary с прозрачностью
      borderColor: '#3b82f6',
      data: [40, 39, 50, 40, 70, 80, 95], // Пример цифр
      fill: true,
      tension: 0.4 // Сглаживание линий
    }
  ]
}

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false } 
  },
  scales: {
    y: { beginAtZero: true, grid: { display: false } },
    x: { grid: { display: false } }
  }
}
</script>

