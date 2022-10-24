<script lang="ts">
import Sidebar from "./lib/Sidebar/Sidebar.svelte";
let sidebarData = [
  {
    title: "CPU",
    link: "/cpu",
    id: "cpu"
  },
  {
    title: "Memory",
    link: "/memory",
    id: "memory"
  },
  {
    title: "Disks",
    link: "/disks",
    id: "disks"
  },
  {
    title: "Devices",
    link: "/devices",
    id: "devices"
  },
  {
    title: "Temperature",
    link: "/temperature",
    id: "temperature"
  }
];

let selected = sidebarData[0];

const onSelectItem=(item)=>{
  selected = item;
}
</script>

<main class="container">
  <div class="main__wrapper">
    <Sidebar onSelectItem={onSelectItem} selected={selected} sidebarData={sidebarData}/>
    <div class="data__container"> </div>
  </div>
</main>

<style>
  .main__wrapper{
    display: grid;
    grid-template-columns: 20% 80%;
    gap: 10px;
    align-items: center;
    align-content: center;
    place-items: center;
    place-content: center;
  }
  .data__container{
    width: 100%;
  }
</style>