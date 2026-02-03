<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import Chart from 'chart.js/auto';
  import 'chartjs-adapter-date-fns';

  let { metrics = [], type = 'cpu' } = $props();

  let canvas: HTMLCanvasElement;
  let chart: Chart;

  function updateChart() {
    if (!chart) return;

    chart.data.labels = metrics.map((m: any) => new Date(m.timestamp));
    chart.data.datasets[0].data = metrics.map((m: any) =>
      type === 'cpu' ? m.cpu_usage : m.memory_usage / (1024 * 1024)
    );
    chart.update('none');
  }

  onMount(() => {
    chart = new Chart(canvas, {
      type: 'line',
      data: {
        labels: metrics.map((m: any) => new Date(m.timestamp)),
        datasets: [{
          label: type === 'cpu' ? 'CPU (%)' : 'Memory (MB)',
          data: metrics.map((m: any) => type === 'cpu' ? m.cpu_usage : m.memory_usage / (1024 * 1024)),
          borderColor: type === 'cpu' ? '#3b82f6' : '#10b981',
          backgroundColor: type === 'cpu' ? '#3b82f622' : '#10b98122',
          fill: true,
          tension: 0.4,
          pointRadius: 0
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: { display: false },
          tooltip: {
            mode: 'index',
            intersect: false,
          }
        },
        scales: {
          x: {
            type: 'time',
            time: {
              unit: 'minute',
              displayFormats: {
                minute: 'HH:mm'
              }
            },
            grid: { display: false }
          },
          y: {
            beginAtZero: true,
            suggestedMax: type === 'cpu' ? 100 : undefined,
            grid: { color: 'rgba(0,0,0,0.05)' }
          }
        }
      }
    });
  });

  onDestroy(() => {
    if (chart) chart.destroy();
  });

  $effect(() => {
    if (metrics) updateChart();
  });
</script>

<div class="w-full h-[150px] mt-2">
  <canvas bind:this={canvas}></canvas>
</div>
