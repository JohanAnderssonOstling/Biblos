<script lang="ts">
    interface TocElem {name: string, value: string, children: TocElem[], collapsed: Boolean}

    import TreeView from './TreeView.svelte'; // Ensure recursive import
    export let items: TocElem[] = [];
    export let onItemClicked: (value: string) => void = () => {};

    function handleItemClick(item: TocElem) {onItemClicked(item.value);}
</script>

<ul>
    {#each items as item}
        <li class="list-item">
            {#if item.children && item.children.length > 0}
                <button on:click={() => {item.collapsed = !item.collapsed}}>
                    {item.collapsed ? '▶' : '▼'}
                </button>
            {/if}
            <span on:click= {() => {handleItemClick(item)}}>{item.name}</span>
            {#if item.children && item.children.length > 0 && !item.collapsed}
                <TreeView items={item.children} onItemClicked = {onItemClicked} > </TreeView>
            {/if}
        </li>
    {/each}
</ul>

<style>
    ul                  {list-style-type: none;padding-left: 1em; padding-right: 1em;margin: 0;}
    li                  {cursor: pointer; margin-right: 4em}
    .list-item:hover    {background-color: rgba(0, 0, 0, 0.1);}

    span {
        white-space: nowrap; /* Prevent text from wrapping */
        overflow: hidden; /* Hide overflowed text */
        text-overflow: ellipsis; /* Show ellipsis for overflowed text */
        display: inline-block;
        width: 100%;
    }
</style>