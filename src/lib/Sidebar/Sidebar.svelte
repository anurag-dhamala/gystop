<script lang="ts">
  export let sidebarData;
  export let selected;
  export let onSelectItem;
</script>

<div class="sidebar__container">
  {#each sidebarData as item}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div
      class={item.id === selected.id
        ? 'sidebar__item__selected'
        : 'sidebar__item'}
      id={item.id}
      on:click={() => onSelectItem(item)}
    >
      {item.title}
    </div>
  {/each}
</div>

<style>
  .sidebar__container {
    display: flex;
    flex-direction: column;
    justify-content: stretch;
    border-right: 1px solid gray;
    gap: 50px;
    width: 100%;
    align-items: stretch;
    height: 80vh;
  }

  .sidebar__item {
    cursor: pointer;
    padding: 10px;
  }

  .sidebar__item__selected {
    font-weight: bold;
    padding: 10px;
  }

  .sidebar__item:hover {
    background: #e4e4e4;
  }
</style>
