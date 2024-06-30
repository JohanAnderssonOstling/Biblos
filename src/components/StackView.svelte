<script lang="ts">
    import { writable } from 'svelte/store';
    import type { Writable } from 'svelte/store';
    import {onMount, type SvelteComponent} from 'svelte';
    import {setContext } from 'svelte';

    export let root: ComponentType | null = null;
    export { push, pop, STACK_CONTEXT_KEY };

    type ComponentType  = typeof SvelteComponent;
    type StackItem      = { component: ComponentType, props: Record<string, any> };

    const stack: Writable<StackItem[]> = writable([]);
    const STACK_CONTEXT_KEY = "stackview";

    onMount(() => {if (root) {push(root);}});
    function push(component: typeof SvelteComponent, props: Record<string, any> = {}) {
        stack.update(s => [...s, {component, props}]);
    }
    function pop() {stack.update(s => s.slice(0, -1));}
    setContext(STACK_CONTEXT_KEY, { push, pop });
</script>

{#if $stack.length > 0}
    <svelte:component this={$stack[$stack.length - 1].component} {...$stack[$stack.length - 1].props} />
{/if}
