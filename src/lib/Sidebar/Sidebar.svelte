<script lang="ts">
    export let sidebarData;
    export let selected;
    export let onSelectItem;
</script>


<div class="sidebar__container">
    {#each sidebarData as item}
        <div
                class={item.id === selected.id ? "sidebar__item__selected": "sidebar__item"}
                id={item.id}
                on:click={()=>onSelectItem(item)}
        >
            {item.title}
        </div>
        {/each}
</div>


<style>
    .sidebar__container{
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        border-right: 1px solid gray;
        gap: 50px;
        width: 100%;
        height: 100%;
    }

    .sidebar__item{
        cursor: pointer;
    }

    .sidebar__item__selected{
        font-weight: bold;
    }

    .sidebar__item:hover{
        background: gray;
    }
</style>