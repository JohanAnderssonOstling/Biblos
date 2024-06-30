<script lang="ts">
import {getContext, onMount} from "svelte";
import {invoke} from "@tauri-apps/api/tauri";
import BookReader from "./BookReader.svelte";
import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
import Book from "../components/BookCover.svelte"
import BookCover from "../components/BookCover.svelte";
import Dir from "../components/Dir.svelte";
const { push ,pop} = getContext("stackview");
let loading = true
export let libPath          : string
export let currentPath      : string
let books: string[]         = [];
let dirs: string[]          = [];
onMount(() => {load_dir(libPath)})

function back() {
    if (currentPath === libPath) {pop(); return;}
    currentPath = currentPath.substring(0, currentPath.lastIndexOf("/"))
    load_dir(currentPath)
}

function load_dir(path: string) {
    loading = true
    invoke("get_library", {path: path} ).then((bookss) => {
        books = bookss[0];      dirs = bookss[1];       loading = false
    })
}

function openDir(path: string) {
    currentPath = path;     load_dir(currentPath)
}
</script>

<div>
    <div class = "topbar">
        <button on:click={back}>Back</button>
    </div>
    {#if loading}
        <h1>Loading</h1>
    {:else}
    <div class = "bookgrid">
        {#each books as book}
            <BookCover book_path={book} libPath={libPath}></BookCover>
        {/each}
    </div>

    {#each dirs as dir}
        <div class = "directory" on:click={() => openDir(dir)}>
            <h1>{dir.split("/").slice(-1)}</h1>
        </div>
    {/each}
    {/if}
</div>

<style>
.bookgrid {
    display: grid;
    grid-gap: 20px;
    margin-left: 20px;
    margin-right: 20px;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
}


</style>