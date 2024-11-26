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



<script lang="js">
	// Internal stack of components
	import {onMount, setContext} from "svelte";

	let components = [];
	let idCounter = 0;
	export let root= null;
	export const STACK_CONTEXT_KEY = "stackview";

	onMount(() => {if (root) {push(root);}});

	// Expose the push method
	export function push(component, props = {}) {
		// Add the new component to the stack
		idCounter += 1;
		components = [...components, { id: idCounter, component, props }];
	}

	// Expose the pop method
	export function pop() {
		// Remove the top component from the stack
		//const popped = components[components.length - 1];
		components = components.slice(0, -1);
		//return popped;
	}
	setContext(STACK_CONTEXT_KEY, { push, pop });
</script>

{#if components.length > 0}
    {#each components as item (item.id)}
        <div style="display: {item === components[components.length - 1] ? 'block' : 'none'};">
            <svelte:component this={item.component} {...item.props} />
        </div>
    {/each}
{/if}
