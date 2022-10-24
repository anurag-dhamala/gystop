<script lang="ts">
  import MainContent from './lib/MainContent/MainContent.svelte';
  import Sidebar from './lib/Sidebar/Sidebar.svelte';
  import { InfoCategory } from './utils/InfoCategory';
  let sidebarData = [
    {
      title: 'CPU',
      link: '/cpu',
      id: InfoCategory.CPU,
    },
    {
      title: 'Memory',
      link: '/memory',
      id: InfoCategory.MEMORY,
    },
    {
      title: 'Disks',
      link: '/disks',
      id: InfoCategory.DISKS,
    },
    {
      title: 'Devices',
      link: '/devices',
      id: InfoCategory.DEVICES,
    },
    {
      title: 'Temperature',
      link: '/temperature',
      id: InfoCategory.TEMPERATURE,
    },
  ];

  let selected = sidebarData[0];

  const onSelectItem = (item) => {
    selected = item;
  };
</script>

<main class="container">
  <div class="main__wrapper">
    <Sidebar {onSelectItem} {selected} {sidebarData} />
    <div class="data__container">
      <MainContent selectedItem={selected} />
    </div>
  </div>
</main>

<style>
  .main__wrapper {
    display: grid;
    grid-template-columns: 20% 80%;
    gap: 10px;
    align-items: center;
    align-content: center;
    place-items: center;
    place-content: center;
    height: 100%;
  }
  .data__container {
    width: 100%;
    height: 100%;
  }
</style>
