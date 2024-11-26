<script lang="ts">
    import TreeView from './TreeView.svelte'; // Ensure recursive import
	interface TocElem {name: string, value: string, children: TocElem[], collapsed: Boolean, current: Boolean}

	export let items: TocElem[] = [];
    export let onItemClicked: (value: string) => void = () => {};

    function handleItemClick(item: TocElem) {onItemClicked(item.value);}
</script>

<ul>
    {#each items as item}
        <li class="list-item">
            {#if item.children && item.children.length > 0}
                <span on:click={() => {item.collapsed = !item.collapsed}}>
                    {item.collapsed ? '▶' : '▼'}
                </span>
            {/if}
            {#if item.current}
                <span class = "toc-text" style = "font-weight: bold"on:click= {() => {handleItemClick(item)}}>{item.name}</span>

                {:else}
            <span class = "toc-text" on:click= {() => {handleItemClick(item)}}>{item.name}</span>
                {/if}
        </li>
        {#if item.children && item.children.length > 0 && !item.collapsed}
            <TreeView items={item.children} onItemClicked = {onItemClicked} > </TreeView>
        {/if}
    {/each}
</ul>

<style>
    ul                  {list-style-type: none;padding-left: 1em;margin:0;width:100%}
    li                  {display: flex}

    span {
        white-space: nowrap; /* Prevent text from wrapping */
        overflow: hidden; /* Hide overflowed text */
        text-overflow: ellipsis; /* Show ellipsis for overflowed text */
        display: inline-block;
        cursor: pointer;
        padding: 0.5em;
    }

    .toc-text {
        width: 100%;
    }
    span:hover {background-color: rgba(0, 0, 0, 0.1); border-radius: 4px}
</style>