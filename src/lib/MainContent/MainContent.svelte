<script lang="ts">
  import { InfoCategory } from '../../utils/InfoCategory';
  import Cpu from './Cpu/Cpu.svelte';
  import Devices from './Devices/Devices.svelte';

  export let selectedItem;
</script>

{#if selectedItem.id === InfoCategory.CPU}
  <Cpu />
{:else if selectedItem === InfoCategory.DEVICES}
  <Devices />
{:else}
  <div></div>
{/if}

<div></div>
