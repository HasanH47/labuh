<script lang="ts">
  let { metrics = [], type = 'cpu' } = $props();

  let dataPoints = $derived(
    metrics.map((m: any, i: number) => ({
      x: i,
      y: type === 'cpu' ? m.cpu_usage : m.memory_usage / (1024 * 1024) // to MB
    }))
  );

  let max_y = $derived(Math.max(...dataPoints.map(p => p.y), type === 'cpu' ? 100 : 512));
  let width = 400;
  let height = 150;

  let points = $derived(
    dataPoints.map((p, i) => {
      const x = (i / (dataPoints.length - 1 || 1)) * width;
      const y = height - (p.y / max_y) * height;
      return `${x},${y}`;
    }).join(' ')
  );
</script>

<div class="w-full h-[150px] relative mt-2">
  <svg {width} {height} viewBox="0 0 {width} {height}" class="w-full h-full overflow-visible">
    <!-- Grid -->
    <line x1="0" y1="0" x2={width} y2="0" stroke="currentColor" stroke-opacity="0.1" />
    <line x1="0" y1={height/2} x2={width} y2={height/2} stroke="currentColor" stroke-opacity="0.1" />
    <line x1="0" y1={height} x2={width} y2={height} stroke="currentColor" stroke-opacity="0.2" />

    <!-- Line -->
    {#if points}
      <polyline
        fill="none"
        stroke={type === 'cpu' ? '#3b82f6' : '#10b981'}
        stroke-width="2"
        {points}
      />
      <!-- Area fill -->
      <polyline
        fill={type === 'cpu' ? '#3b82f622' : '#10b98122'}
        points="0,{height} {points} {width},{height}"
      />
    {/if}
  </svg>

  <div class="absolute top-0 right-0 text-[10px] text-muted-foreground">
    Max: {max_y.toFixed(1)} {type === 'cpu' ? '%' : 'MB'}
  </div>
</div>
